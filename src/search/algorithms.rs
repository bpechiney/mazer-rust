use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum SearchAlgorithm {
    /// The A* search algorithm.
    ///
    /// See [this article](https://tinyurl.com/yc55tskf) for more information.
    AStar,
    /// The breadth-first search algorithm.
    ///
    /// See [this article](https://tinyurl.com/ysujxksz) for more information.
    BreadthFirst,
    /// The depth-first search algorithm.
    ///
    /// See [this article](https://tinyurl.com/yuxurrdu) for more information.
    DepthFirst,
    /// Dijkstra's algorithm.
    ///
    /// See [this article](https://tinyurl.com/343hhkaa) for more information.
    Dijkstras,
}
