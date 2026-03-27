# Legal Entity Dissolution Auto-Pause Feature

## Overview

This feature implements Issue #208: Support for Legal Entity Dissolution Auto-Pause. It provides an "Entity Monitor" hook that automatically freezes all active grant streams when a company receiving grants is reported as dissolved by an authorized Legal Oracle.

## Problem Statement

When a company receiving grant funding goes bankrupt or is dissolved, the DAO's treasury is at risk of continuing to send funds to a "legal void" - an entity that can no longer legally receive or use the funds. This creates both financial risk and regulatory compliance issues.

## Solution

The solution introduces a Legal Entity Monitor system that:

1. **Legal Oracle Integration**: An authorized Legal Oracle contract can report entity status changes
2. **Auto-Pause Mechanism**: When dissolution is reported, all active streams to that entity are automatically paused
3. **Status Caching**: Entity status is cached to optimize performance and reduce oracle calls
4. **Audit Trail**: All dissolution events are logged with full audit trail

## Architecture

### Core Components

#### 1. Legal Entity Status Types
```rust
pub enum LegalEntityStatus {
    Active,           // Entity is in good standing
    Dissolved,        // Entity has been dissolved
    Inactive,         // Entity is temporarily inactive
    Suspended,        // Entity is under suspension
    Bankrupt,         // Entity has declared bankruptcy
}
```

#### 2. Entity Status Cache
```rust
pub struct EntityStatusCache {
    pub status: LegalEntityStatus,
    pub last_updated: u64,
    pub source: Address, // Legal Oracle that provided this status
}
```

#### 3. Dissolution Event Record
```rust
pub struct LegalEntityDissolutionEvent {
    pub entity_address: Address,
    pub dissolution_timestamp: u64,
    pub reported_by: Address, // Legal Oracle address
    pub affected_grants: Vec<u64>, // List of grant IDs that were paused
}
```

### Key Functions

#### Admin Functions
- `set_legal_oracle_contract(address)` - Configure authorized Legal Oracle

#### Legal Oracle Functions
- `report_entity_dissolution(entity, timestamp, evidence)` - Report entity dissolution
- `update_entity_status(entity, status, evidence)` - Update entity to any status

#### Public Query Functions
- `get_entity_status(entity)` - Get current entity status
- `get_dissolved_entities()` - Get list of all dissolved entities
- `is_entity_dissolved(entity)` - Check if entity is dissolved
- `get_legal_oracle_contract()` - Get authorized Legal Oracle address

## Security Features

### Authorization
- Only admin can set the Legal Oracle contract address
- Only authorized Legal Oracle can report entity status changes
- All oracle calls are verified against the stored oracle address

### Data Integrity
- Entity status is stored permanently with audit trail
- Status caching with 24-hour expiration for performance
- All status changes emit events for off-chain monitoring

### Protection Against Attacks
- Duplicate dissolution reports are rejected
- Unauthorized oracle calls are rejected
- All state changes are atomic

## Workflow

### 1. Setup (Admin)
1. Admin calls `set_legal_oracle_contract()` with the authorized Legal Oracle address
2. Contract emits `legal_oracle_set` event

### 2. Entity Dissolution Reporting (Legal Oracle)
1. Legal Oracle detects entity dissolution from business registry
2. Oracle calls `report_entity_dissolution()` with entity details and evidence
3. Contract verifies oracle authorization
4. Contract updates entity status to `Dissolved`
5. Contract finds all active grants for the entity
6. Contract pauses all active grants
7. Contract emits `entity_dissolved` and `auto_paused` events
8. Contract returns dissolution event details

### 3. Status Queries (Public)
1. Anyone can query entity status via `get_entity_status()`
2. Cached status is returned if valid (within 24 hours)
3. Dissolved entities list available via `get_dissolved_entities()`

## Events

### legal_oracle_set
Emitted when admin sets the Legal Oracle contract address.
- `oracle_address: Address`

### entity_dissolved
Emitted when an entity dissolution is reported.
- `entity_address: Address`
- `dissolution_timestamp: u64`
- `reported_by: Address`
- `affected_grants_count: u32`

### auto_paused
Emitted for each grant that is automatically paused.
- `grant_id: u64`
- `entity_address: Address`
- `dissolution_timestamp: u64`

### entity_status_updated
Emitted when entity status is updated.
- `entity_address: Address`
- `new_status: LegalEntityStatus`
- `updated_by: Address`
- `timestamp: u64`

## Gas Optimization

### Caching Strategy
- Entity status is cached for 24 hours to reduce oracle calls
- Cache includes source oracle and timestamp for validation
- Cached status is used for public queries

### Batch Operations
- All grants for an entity are processed in a single transaction
- Multiple grants can be paused atomically

### Storage Optimization
- Dissolved entities stored in single list
- Status cache uses minimal storage per entity

## Testing

Comprehensive test suite covers:
- Legal Oracle setup and authorization
- Entity dissolution reporting and auto-pause
- Multiple grants handling
- Status validation and caching
- Error conditions (unauthorized access, duplicate reports)
- Edge cases (already paused grants, different entity statuses)

## Integration Points

### Legal Oracle Contract
The system expects a Legal Oracle contract that:
- Connects to official business registries
- Provides entity status information
- Can be called to verify entity dissolution
- Maintains its own authorization and security

### Grant System Integration
- Integrates with existing grant pause/resume functionality
- Uses existing grant storage and status system
- Leverages existing event system for notifications

### Off-Chain Monitoring
- Events can be monitored by off-chain systems
- Audit trail available for compliance reporting
- Real-time notifications for treasury management

## Future Enhancements

### Automatic Recovery
- Automatic grant resumption if entity status changes back to Active
- Treasury recovery procedures for dissolved entities

### Multi-Jurisdiction Support
- Support for different legal jurisdictions
- Jurisdiction-specific dissolution rules

### Advanced Filtering
- Filter grants by type, amount, or other criteria
- Selective pause based on grant characteristics

## Deployment Considerations

### Configuration
- Legal Oracle contract address must be set by admin
- Cache duration can be adjusted if needed
- Event monitoring should be set up off-chain

### Security
- Legal Oracle contract must be thoroughly audited
- Admin keys must be securely managed
- Event monitoring for suspicious activity

### Monitoring
- Monitor for dissolution events
- Track treasury impact of paused grants
- Audit oracle reporting patterns

## Conclusion

This feature provides robust protection for the DAO treasury by automatically pausing grants to dissolved entities. It maintains security, provides comprehensive audit trails, and integrates seamlessly with the existing grant system while enabling future enhancements.
