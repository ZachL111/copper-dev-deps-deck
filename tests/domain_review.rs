use copper_dev_deps_deck::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 57, slack: 42, drag: 23, confidence: 56 };
    assert_eq!(review_score(case), 143);
    assert_eq!(review_lane(case), "ship");
}
