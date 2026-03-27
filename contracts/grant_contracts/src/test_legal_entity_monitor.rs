#[cfg(test)]
mod test_legal_entity_monitor {
    use super::*;
    use soroban_sdk::testutils::{Address as TestAddress, Ledger as TestLedger};
    use soroban_sdk::{symbol_short, vec};

    #[test]
    fn test_legal_oracle_setup() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Verify legal oracle is set
        let retrieved_oracle = GrantContract::get_legal_oracle_contract(env.clone()).unwrap();
        assert_eq!(retrieved_oracle, legal_oracle);
    }

    #[test]
    fn test_entity_dissolution_auto_pause() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let entity = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Create a grant for the entity
        let grant_id = 1;
        let grant = Grant {
            recipient: entity.clone(),
            total_amount: 1000,
            withdrawn: 0,
            claimable: 0,
            flow_rate: 100,
            base_flow_rate: 100,
            last_update_ts: env.ledger().timestamp(),
            rate_updated_at: env.ledger().timestamp(),
            last_claim_time: env.ledger().timestamp(),
            pending_rate: 0,
            effective_timestamp: 0,
            status: GrantStatus::Active,
            redirect: None,
            stream_type: StreamType::FixedAmount,
            start_time: env.ledger().timestamp(),
            warmup_duration: 0,
            required_stake: 0,
            staked_amount: 0,
            stake_token: token.clone(),
            slash_reason: None,
            lessor: TestAddress::random(&env),
            property_id: String::from_str(&env, "property1"),
            serial_number: String::from_str(&env, "SN001"),
            security_deposit: 100,
            lease_end_time: env.ledger().timestamp() + 1000,
            validator: TestAddress::random(&env),
            validator_withdrawn: 0,
            validator_claimable: 0,
            linked_addresses: Vec::new(&env),
            milestone_amount: 0,
            total_milestones: 0,
            claimed_milestones: 0,
            available_milestone_funds: 0,
        };

        // Store the grant
        env.storage().instance().set(&DataKey::Grant(grant_id), &grant);
        
        // Add grant ID to list
        let mut grant_ids = Vec::new(&env);
        grant_ids.push_back(grant_id);
        env.storage().instance().set(&DataKey::GrantIds, &grant_ids);

        // Verify grant is initially active
        let stored_grant = read_grant(&env, grant_id).unwrap();
        assert_eq!(stored_grant.status, GrantStatus::Active);

        // Report entity dissolution
        let dissolution_timestamp = env.ledger().timestamp();
        let evidence = String::from_str(&env, "Company dissolved by court order");
        
        let dissolution_event = GrantContract::report_entity_dissolution(
            env.clone(),
            entity.clone(),
            dissolution_timestamp,
            evidence.clone(),
        ).unwrap();

        // Verify dissolution event details
        assert_eq!(dissolution_event.entity_address, entity);
        assert_eq!(dissolution_event.dissolution_timestamp, dissolution_timestamp);
        assert_eq!(dissolution_event.reported_by, legal_oracle);
        assert_eq!(dissolution_event.affected_grants.len(), 1);
        assert_eq!(dissolution_event.affected_grants.get(0).unwrap(), &grant_id);

        // Verify grant is now paused
        let paused_grant = read_grant(&env, grant_id).unwrap();
        assert_eq!(paused_grant.status, GrantStatus::Paused);

        // Verify entity status is dissolved
        let entity_status = GrantContract::get_entity_status(env.clone(), entity.clone()).unwrap();
        assert_eq!(entity_status, LegalEntityStatus::Dissolved);

        // Verify entity is in dissolved list
        let dissolved_entities = GrantContract::get_dissolved_entities(env.clone());
        assert_eq!(dissolved_entities.len(), 1);
        assert_eq!(dissolved_entities.get(0).unwrap(), &entity);
    }

    #[test]
    fn test_multiple_grants_auto_pause() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let entity = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Create multiple grants for the same entity
        let grant_ids = vec![&env, 1, 2, 3];
        let mut grant_id_list = Vec::new(&env);

        for &grant_id in grant_ids.iter() {
            let grant = Grant {
                recipient: entity.clone(),
                total_amount: 1000,
                withdrawn: 0,
                claimable: 0,
                flow_rate: 100,
                base_flow_rate: 100,
                last_update_ts: env.ledger().timestamp(),
                rate_updated_at: env.ledger().timestamp(),
                last_claim_time: env.ledger().timestamp(),
                pending_rate: 0,
                effective_timestamp: 0,
                status: GrantStatus::Active,
                redirect: None,
                stream_type: StreamType::FixedAmount,
                start_time: env.ledger().timestamp(),
                warmup_duration: 0,
                required_stake: 0,
                staked_amount: 0,
                stake_token: token.clone(),
                slash_reason: None,
                lessor: TestAddress::random(&env),
                property_id: String::from_str(&env, "property1"),
                serial_number: String::from_str(&env, "SN001"),
                security_deposit: 100,
                lease_end_time: env.ledger().timestamp() + 1000,
                validator: TestAddress::random(&env),
                validator_withdrawn: 0,
                validator_withdrawn: 0,
                linked_addresses: Vec::new(&env),
                milestone_amount: 0,
                total_milestones: 0,
                claimed_milestones: 0,
                available_milestone_funds: 0,
            };

            env.storage().instance().set(&DataKey::Grant(grant_id), &grant);
            grant_id_list.push_back(grant_id);
        }

        // Store grant IDs
        env.storage().instance().set(&DataKey::GrantIds, &grant_id_list);

        // Report entity dissolution
        let dissolution_timestamp = env.ledger().timestamp();
        let evidence = String::from_str(&env, "Company dissolved by court order");
        
        let dissolution_event = GrantContract::report_entity_dissolution(
            env.clone(),
            entity.clone(),
            dissolution_timestamp,
            evidence.clone(),
        ).unwrap();

        // Verify all grants are paused
        assert_eq!(dissolution_event.affected_grants.len(), 3);
        for &grant_id in grant_ids.iter() {
            let paused_grant = read_grant(&env, grant_id).unwrap();
            assert_eq!(paused_grant.status, GrantStatus::Paused);
            
            // Verify grant is in affected list
            assert!(dissolution_event.affected_grants.contains(&grant_id));
        }
    }

    #[test]
    fn test_only_active_grants_paused() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let entity = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Create grants with different statuses
        let active_grant = Grant {
            recipient: entity.clone(),
            total_amount: 1000,
            withdrawn: 0,
            claimable: 0,
            flow_rate: 100,
            base_flow_rate: 100,
            last_update_ts: env.ledger().timestamp(),
            rate_updated_at: env.ledger().timestamp(),
            last_claim_time: env.ledger().timestamp(),
            pending_rate: 0,
            effective_timestamp: 0,
            status: GrantStatus::Active,
            redirect: None,
            stream_type: StreamType::FixedAmount,
            start_time: env.ledger().timestamp(),
            warmup_duration: 0,
            required_stake: 0,
            staked_amount: 0,
            stake_token: token.clone(),
            slash_reason: None,
            lessor: TestAddress::random(&env),
            property_id: String::from_str(&env, "property1"),
            serial_number: String::from_str(&env, "SN001"),
            security_deposit: 100,
            lease_end_time: env.ledger().timestamp() + 1000,
            validator: TestAddress::random(&env),
            validator_withdrawn: 0,
            validator_claimable: 0,
            linked_addresses: Vec::new(&env),
            milestone_amount: 0,
            total_milestones: 0,
            claimed_milestones: 0,
            available_milestone_funds: 0,
        };

        let paused_grant = Grant {
            status: GrantStatus::Paused,
            ..active_grant.clone()
        };

        let completed_grant = Grant {
            status: GrantStatus::Completed,
            ..active_grant.clone()
        };

        // Store grants
        env.storage().instance().set(&DataKey::Grant(1), &active_grant);
        env.storage().instance().set(&DataKey::Grant(2), &paused_grant);
        env.storage().instance().set(&DataKey::Grant(3), &completed_grant);

        let mut grant_ids = Vec::new(&env);
        grant_ids.push_back(1);
        grant_ids.push_back(2);
        grant_ids.push_back(3);
        env.storage().instance().set(&DataKey::GrantIds, &grant_ids);

        // Report entity dissolution
        let dissolution_timestamp = env.ledger().timestamp();
        let evidence = String::from_str(&env, "Company dissolved by court order");
        
        let dissolution_event = GrantContract::report_entity_dissolution(
            env.clone(),
            entity.clone(),
            dissolution_timestamp,
            evidence.clone(),
        ).unwrap();

        // Verify only active grant was paused
        assert_eq!(dissolution_event.affected_grants.len(), 1);
        assert_eq!(dissolution_event.affected_grants.get(0).unwrap(), &1);

        // Verify statuses
        let grant1 = read_grant(&env, 1).unwrap();
        assert_eq!(grant1.status, GrantStatus::Paused);

        let grant2 = read_grant(&env, 2).unwrap();
        assert_eq!(grant2.status, GrantStatus::Paused); // Already paused, remains paused

        let grant3 = read_grant(&env, 3).unwrap();
        assert_eq!(grant3.status, GrantStatus::Completed); // Unchanged
    }

    #[test]
    fn test_unauthorized_legal_oracle() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let unauthorized_oracle = TestAddress::random(&env);
        let entity = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Try to report dissolution with unauthorized oracle
        let dissolution_timestamp = env.ledger().timestamp();
        let evidence = String::from_str(&env, "Company dissolved by court order");
        
        let result = GrantContract::report_entity_dissolution(
            env.clone(),
            entity.clone(),
            dissolution_timestamp,
            evidence.clone(),
        );

        // Should fail with unauthorized error
        assert_eq!(result.unwrap_err(), Error::UnauthorizedLegalOracle);
    }

    #[test]
    fn test_duplicate_dissolution_report() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let entity = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Report entity dissolution first time
        let dissolution_timestamp = env.ledger().timestamp();
        let evidence = String::from_str(&env, "Company dissolved by court order");
        
        let result1 = GrantContract::report_entity_dissolution(
            env.clone(),
            entity.clone(),
            dissolution_timestamp,
            evidence.clone(),
        );
        assert!(result1.is_ok());

        // Try to report dissolution second time
        let result2 = GrantContract::report_entity_dissolution(
            env.clone(),
            entity.clone(),
            dissolution_timestamp,
            evidence.clone(),
        );

        // Should fail with already dissolved error
        assert_eq!(result2.unwrap_err(), Error::EntityAlreadyDissolved);
    }

    #[test]
    fn test_entity_status_cache() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let entity = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Report entity dissolution
        let dissolution_timestamp = env.ledger().timestamp();
        let evidence = String::from_str(&env, "Company dissolved by court order");
        
        GrantContract::report_entity_dissolution(
            env.clone(),
            entity.clone(),
            dissolution_timestamp,
            evidence.clone(),
        ).unwrap();

        // Check status from cache
        let status = GrantContract::get_entity_status(env.clone(), entity.clone()).unwrap();
        assert_eq!(status, LegalEntityStatus::Dissolved);

        // Check cache validity
        assert!(is_entity_status_cache_valid(&env, &entity));

        // Check if entity is dissolved
        assert!(GrantContract::is_entity_dissolved(env.clone(), entity.clone()));
    }

    #[test]
    fn test_update_entity_status() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = TestAddress::random(&env);
        let legal_oracle = TestAddress::random(&env);
        let entity = TestAddress::random(&env);
        let token = TestAddress::random(&env);
        let treasury = TestAddress::random(&env);
        let oracle = TestAddress::random(&env);
        let native_token = TestAddress::random(&env);

        // Initialize contract
        GrantContract::initialize(
            env.clone(),
            admin.clone(),
            token.clone(),
            treasury.clone(),
            oracle.clone(),
            native_token.clone(),
        ).unwrap();

        // Set legal oracle
        GrantContract::set_legal_oracle_contract(env.clone(), legal_oracle.clone()).unwrap();

        // Update entity status to suspended
        let evidence = String::from_str(&env, "Company temporarily suspended");
        
        let result = GrantContract::update_entity_status(
            env.clone(),
            entity.clone(),
            LegalEntityStatus::Suspended,
            evidence.clone(),
        );
        assert!(result.is_ok());

        // Verify status is updated
        let status = GrantContract::get_entity_status(env.clone(), entity.clone()).unwrap();
        assert_eq!(status, LegalEntityStatus::Suspended);

        // Verify entity is not in dissolved list
        let dissolved_entities = GrantContract::get_dissolved_entities(env.clone());
        assert_eq!(dissolved_entities.len(), 0);

        // Update to dissolved status
        let evidence2 = String::from_str(&env, "Company now dissolved");
        
        let result2 = GrantContract::update_entity_status(
            env.clone(),
            entity.clone(),
            LegalEntityStatus::Dissolved,
            evidence2.clone(),
        );
        assert!(result2.is_ok());

        // Verify entity is now in dissolved list
        let dissolved_entities2 = GrantContract::get_dissolved_entities(env.clone());
        assert_eq!(dissolved_entities2.len(), 1);
        assert_eq!(dissolved_entities2.get(0).unwrap(), &entity);
    }
}
