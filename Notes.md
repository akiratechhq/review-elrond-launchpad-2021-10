# Notes

Files:

- [ ] launchpad.rs
  - [ ] claim_ticket_payment
  - [ ] force_claim_period_start
  - [ ] add_address_to_blacklist
  - [ ] remove_address_from_blacklist
  - [ ] refund_confirmed_tickets
  - [ ] add_tickets
  - [ ] select_winners
  - [ ] confirm_tickets
  - [ ] select_new_winners
  - [ ] claim_launchpad_tokens
  - [ ] get_ticket_range_for_address
  - [ ] get_winning_ticket_ids_for_address
  - [ ] get_confirmed_ticket_ids_for_address
  - [ ] get_number_of_winning_tickets_for_address
  - [ ] get_number_of_confirmed_tickets_for_address
  - [ ] get_launch_stage
  - [ ] try_create_tickets
  - [ ] shuffle_single_ticket
  - [ ] swap
  - [ ] start_confirmation_period
  - [ ] try_get_ticket_range
  - [x] get_total_tickets
  - [x] get_total_confirmed_tickets
  - [x] require_stage
  - [x] get_tickets_with_status
    - Assumes the tickets are grouped for a single address/account
  - [x] ticket_status
  - [x] ticket_range_for_address
  - [x] winning_tickets_range
  - [x] shuffled_tickets
  - [x] current_generation
    - Limited to `u8`, 256
  - [x] total_confirmed_tickets
  - [x] blacklist

- [ ] launch_stage.rs
- [ ] ongoing_operation.rs
- [ ] random.rs
- [ ] setup.rs
- [ ] ticket_status.rs