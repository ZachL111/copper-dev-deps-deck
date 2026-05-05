use copper_dev_deps_deck::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 93, capacity: 87, latency: 18, risk: 25, weight: 6 };
    assert_eq!(score(signal), 123);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 100, capacity: 92, latency: 18, risk: 17, weight: 13 };
    assert_eq!(score(signal), 232);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 89, capacity: 71, latency: 17, risk: 9, weight: 4 };
    assert_eq!(score(signal), 185);
    assert_eq!(classify(signal), "accept");
}
