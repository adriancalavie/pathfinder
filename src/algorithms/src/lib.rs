mod random;
pub mod utils;

pub fn get_random_data() -> utils::Map {
    let mut rng = random::RandomEngine::new(rand::thread_rng(), 0. ..1.);

    let points = vec![
        utils::Point::new(rng.get_x(), rng.get_y()),
        utils::Point::new(rng.get_x(), rng.get_y()),
        utils::Point::new(rng.get_x(), rng.get_y()),
        utils::Point::new(rng.get_x(), rng.get_y()),
        utils::Point::new(rng.get_x(), rng.get_y()),
    ];

    utils::Map::new(vec![
        utils::Node::new(points[0], vec![points[1]]),
        utils::Node::new(points[1], vec![points[2]]),
        utils::Node::new(points[2], vec![points[3]]),
        utils::Node::new(points[3], vec![points[4]]),
        utils::Node::new(points[4], vec![points[0]]),
    ])
}
