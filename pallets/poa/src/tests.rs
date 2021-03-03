use super::*;
use frame_support::{
    assert_noop, assert_ok,
};
use mock::*;
use super::Event;

#[test]
fn basic_setup_works() {
    // Verifies initial conditions of mock
    ExtBuilder::default().build_and_execute(|| {
        assert_eq!(PoA::authorities(1), Some(AuthorityState::Working));
        assert_eq!(PoA::authorities(2), Some(AuthorityState::Working));
        assert_eq!(PoA::minimum_authority_count(), 2);
        assert_eq!(PoA::history_depth(), 24);
        assert_eq!(PoA::invulnerables(), vec![1, 2]);
        assert_eq!(current_era(), 1);
        assert_eq!(active_era(), 0);

        assert_eq!(Balances::free_balance(1), 1000);
        assert_eq!(Balances::free_balance(2), 1000);

        // New era is not being forced
        assert_eq!(PoA::force_era(), Forcing::NotForcing);
    });
}

#[test]
fn add_authority_should_work() {
    ExtBuilder::default().build_and_execute(|| {
        assert_noop!(
            PoA::add_authority(Origin::root(), 1),
            Error::<Test>::Existed
        );

        assert_ok!(PoA::add_authority(Origin::root(), 3));

        assert_eq!(PoA::authorities_map().len(), 3);
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
    });
}

#[test]
fn remove_authority_should_work() {
    ExtBuilder::default().build_and_execute(|| {
        assert_noop!(
            PoA::remove_authority(Origin::root(), 1),
            Error::<Test>::TooLowAuthorities
        );

        assert_ok!(PoA::add_authority(Origin::root(), 3));

        assert_eq!(PoA::authorities_map().len(), 3);
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));

        assert_noop!(
            PoA::remove_authority(Origin::root(), 4),
            Error::<Test>::NotExisted
        );

        assert_ok!(PoA::remove_authority(Origin::root(), 1));
        assert_eq!(PoA::authorities_map().len(), 2);
    });
}

#[test]
fn set_invulnerables_should_work() {
    ExtBuilder::default().build_and_execute(|| {
        assert_eq!(PoA::invulnerables(), vec![1, 2]);

        assert_ok!(PoA::set_invulnerables(Origin::root(), vec![3]));
        assert_eq!(PoA::invulnerables(), vec![3]);
    });
}

#[test]
fn can_not_slash_invulnerable() {
    ExtBuilder::default().build_and_execute(|| {
        mock::start_active_era(1);

        add_slash(&2);
        // can not slash invulnerable
        assert_eq!(PoA::authorities(2), Some(AuthorityState::Working));
    })
}

#[test]
fn remove_deferred() {
    ExtBuilder::default().build_and_execute(|| {
        mock::start_active_era(1);

        assert_ok!(PoA::add_authority(Origin::root(), 3));
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
        assert_eq!(PoA::authorities_map().len(), 3);

        add_slash(&3);
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
        assert_eq!(PoA::authorities_map().len(), 3);

        mock::start_active_era(2);
        add_slash(&3);
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
        assert_eq!(PoA::authorities_map().len(), 3);

        // fails if empty
        assert_noop!(
            PoA::cancel_deferred_slash(Origin::root(), 1, vec![]),
            Error::<Test>::EmptyTargets
        );

        // fails if not sorted
        assert_noop!(
            PoA::cancel_deferred_slash(Origin::root(), 1, vec![1, 0]),
            Error::<Test>::NotSortedAndUnique
        );

        // authority 3 still waiting
        assert_ok!(PoA::cancel_deferred_slash(Origin::root(), 1, vec![0]));
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
        assert_eq!(PoA::authorities_map().len(), 3);

        mock::start_active_era(3);
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
        assert_eq!(PoA::authorities_map().len(), 3);

        // at the start of era 4, slashes from era 1 are processed,
        // after being deferred for at least 2 full eras.
        mock::start_active_era(4);
        // the first slash was cancelled, so no effect.
        assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
        assert_eq!(PoA::authorities_map().len(), 3);

        mock::start_active_era(5);
        // slash processed now.
        assert_eq!(PoA::authorities(3), None);
        assert_eq!(PoA::authorities_map().len(), 2);
        assert_eq!(
            *mock::poa_events().last().unwrap(),
            Event::Slash(3)
        );
    })
}

#[test]
fn keep_minimum_authority_count() {
    ExtBuilder::default()
        .minimum_authority_count(3)
        .initial_authorities(vec![1, 2, 3])
        .build_and_execute(|| {
            mock::start_active_era(1);

            assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
            assert_eq!(PoA::authorities_map().len(), 3);

            add_slash(&3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            mock::start_active_era(3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            // can not remove due to minimal authority count
            mock::start_active_era(4);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);
            assert_eq!(
                *mock::poa_events().last().unwrap(),
                Event::TooLowAuthorities
            );
    });
}

#[test]
fn slash_multiple_fails_by_limit() {
    ExtBuilder::default()
        .initial_authorities(vec![1, 2, 3])
        .build_and_execute(|| {
            mock::start_active_era(1);

            assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
            assert_eq!(PoA::authorities_map().len(), 3);

            add_slash(&3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            mock::start_active_era(3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            add_slash(&3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            // first slash
            mock::start_active_era(4);
            assert_eq!(PoA::authorities(3), None);
            assert_eq!(PoA::authorities_map().len(), 2);
            assert_eq!(
                *mock::poa_events().last().unwrap(),
                Event::Slash(3)
            );

            // second slash, but limit minimal authority count
            mock::start_active_era(6);
            assert_eq!(PoA::authorities(3), None);
            assert_eq!(PoA::authorities_map().len(), 2);
            assert_eq!(
                *mock::poa_events().last().unwrap(),
                Event::TooLowAuthorities
            );
        });
}

#[test]
fn slash_multiple_should_work() {
    ExtBuilder::default()
        .initial_authorities(vec![1, 2, 3, 4])
        .build_and_execute(|| {
            mock::start_active_era(1);

            assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
            assert_eq!(PoA::authorities_map().len(), 4);

            add_slash(&3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 4);

            mock::start_active_era(3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 4);

            add_slash(&3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 4);

            // first slash
            mock::start_active_era(4);
            assert_eq!(PoA::authorities(3), None);
            assert_eq!(PoA::authorities_map().len(), 3);
            assert_eq!(
                *mock::poa_events().last().unwrap(),
                Event::Slash(3)
            );

            // second slash
            mock::start_active_era(6);
            assert_eq!(PoA::authorities(3), None);
            assert_eq!(PoA::authorities_map().len(), 3);
            assert_eq!(
                *mock::poa_events().last().unwrap(),
                Event::Slash(3)
            );
        });
}

#[test]
fn slash_multiple_state_check() {
    ExtBuilder::default()
        .initial_authorities(vec![1, 2, 3])
        .build_and_execute(|| {
            mock::start_active_era(1);

            assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
            assert_eq!(PoA::authorities_map().len(), 3);

            add_slash(&3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);
            assert_eq!(
                PoA::unapplied_slashes(1),
                vec![UnappliedSlash{ validator: 3u64, reporters: vec![] }]
            );

            mock::start_active_era(2);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            add_slash(&3);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);
            assert_eq!(
                PoA::unapplied_slashes(2),
                vec![UnappliedSlash{ validator: 3u64, reporters: vec![] }]
            );

            // cancel first slash
            mock::start_active_era(3);
            assert_ok!(PoA::cancel_deferred_slash(Origin::root(), 1, vec![0]));
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            // first slash be canceled
            mock::start_active_era(4);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Waiting));
            assert_eq!(PoA::authorities_map().len(), 3);

            // cancel second slash
            assert_ok!(PoA::cancel_deferred_slash(Origin::root(), 2, vec![0]));
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
            assert_eq!(PoA::authorities_map().len(), 3);

            // second slash be canceled
            mock::start_active_era(5);
            assert_eq!(PoA::authorities(3), Some(AuthorityState::Working));
            assert_eq!(PoA::authorities_map().len(), 3);
        });
}
