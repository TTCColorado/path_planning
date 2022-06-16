use geo::algorithm::simplify::Simplify;
use geo::{LineString, Point, Polygon};
use pathplanning::rrt;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;

#[derive(Clone, Copy)]
#[pyclass(module = "path_planning")]
struct SpaceConf {
    bounds: Vec<(f64, f64)>,
    obstacles: Vec<Vec<(f64, f64)>>,
}

#[pymethods]
impl SpaceConf {
    #[new]
    fn new(bounds: Vec<(f64, f64)>, obstacles: Vec<Vec<(f64, f64)>>) -> Self {
        SpaceConf { bounds, obstacles }
    }
}

#[derive(Clone, Copy)]
#[pyclass(module = "path_planning")]
struct RobotConf {
    width: f64,
    height: f64,
    max_steer: f64,
}

#[pymethods]
impl RobotConf {
    #[new]
    fn new(width: f64, height: f64, max_steer: f64) -> Self {
        RobotConf {
            width,
            height,
            max_steer,
        }
    }
}

#[pyclass(module = "path_planning")]
struct PlannerFuture {
    tx: Sender<Option<Vec<(f64, f64)>>>,
    rx: Receiver<Option<Vec<(f64, f64)>>>,
}

impl PlannerFuture {
    fn new(tx: Sender<Option<Vec<(f64, f64)>>>, rx: Receiver<Option<Vec<(f64, f64)>>>) -> Self {
        PlannerFuture { tx, rx }
    }
}

#[pymethods]
impl PlannerFuture {
    fn check(&self) -> PyResult<Option<Vec<(f64, f64)>>> {
        match self.rx.try_recv() {
            Ok(result) => Ok(result),
            Err(e) => match e {
                TryRecvError::Empty => Ok(None),
                TryRecvError::Disconnected => Err(exceptions::Exception::py_err(
                    "Channel to worker thread disconnected",
                )),
            },
        }
    }

    fn is_done(&self, result: Option<Vec<(f64, f64)>>) -> PyResult<bool> {
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn finalize(&self, result: Option<Vec<(f64, f64)>>) -> PyResult<Vec<(f64, f64)>> {
        match result {
            Some(r) => Ok(r),
            None => Err(exceptions::Exception::py_err("Planner failed to find path")),
        }
    }
}

#[pyclass(module = "path_planning")]
struct RRTDubinsPlanner {
    start: (f64, f64),
    start_yaw: f64,
    goal: (f64, f64),
    goal_yaw: f64,
    max_iter: usize,
    step_size: f64,
    space: SpaceConf,
    robot: RobotConf,
}

#[pymethods]
impl RRTDubinsPlanner {
    #[new]
    fn new(
        start: (f64, f64),
        start_yaw: f64,
        goal: (f64, f64),
        goal_yaw: f64,
        max_iter: usize,
        step_size: f64,
        space: SpaceConf,
        robot: RobotConf,
    ) -> Self {
        RRTDubinsPlanner {
            start,
            start_yaw,
            goal,
            goal_yaw,
            max_iter,
            step_size,
            space,
            robot,
        }
    }

    fn plan_async(&self) -> PyResult<PlannerFuture> {
        let robot = rrt::Robot::new(self.robot.width, self.robot.height, self.robot.max_steer);
        let bounds = Polygon::new(LineString::from(self.space.bounds.clone()), vec![]);
        let obs: Vec<Polygon<f64>> = self
            .space
            .obstacles
            .iter()
            .map(|o| Polygon::new(LineString::from(o.clone()), vec![]))
            .collect();
        let space = rrt::Space::new(bounds, robot, obs);
        let planner = Arc::new(rrt::RRT::new(
            self.start.into(),
            self.start_yaw,
            self.goal.into(),
            self.goal_yaw,
            self.max_iter,
            self.step_size,
            space,
        ));

        let (tx, rx) = channel();
        let tx_moved = tx.clone();
        thread::spawn(move || {
            let result = match planner.plan() {
                Some(path) => Some(
                    path
                        // .simplify(&0.01)
                        .points_iter()
                        .map(|p| p.x_y())
                        .collect(),
                ),
                None => None,
            };
            tx_moved
                .send(result)
                .expect("Should send result over channel");
        });

        Ok(PlannerFuture::new(tx, rx))
    }

    fn plan(&self) -> PyResult<Vec<(f64, f64)>> {
        let robot = rrt::Robot::new(self.robot.width, self.robot.height, self.robot.max_steer);
        let bounds = Polygon::new(LineString::from(self.space.bounds.clone()), vec![]);
        let obs: Vec<Polygon<f64>> = self
            .space
            .obstacles
            .iter()
            .map(|o| Polygon::new(LineString::from(o.clone()), vec![]))
            .collect();
        let space = rrt::Space::new(bounds, robot, obs);
        let planner = Arc::new(rrt::RRT::new(
            self.start.into(),
            self.start_yaw,
            self.goal.into(),
            self.goal_yaw,
            self.max_iter,
            self.step_size,
            space,
        ));

        match planner.plan() {
            Some(path) => Ok(path
                // .simplify(&0.01)
                .points_iter()
                .map(|p| p.x_y())
                .collect()),
            None => Err(exceptions::Exception::py_err("Planner failed to find path")),
        }
    }
}

#[pyfunction]
fn create_circle(xy: (f64, f64), r: f64) -> Vec<(f64, f64)> {
    let circle = rrt::create_circle(Point::from(xy), r);
    circle.exterior().points_iter().map(|p| p.x_y()).collect()
}

#[pyfunction]
fn simplify(points: Vec<(f64, f64)>, e: f64) -> Vec<(f64, f64)> {
    let p = points.into_iter().map(|p| p.into()).collect();
    let line = LineString(p);
    line.simplify(&e).points_iter().map(|p| p.x_y()).collect()
}

#[pymodule]
fn path_planning(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(create_circle))?;
    m.add_wrapped(wrap_pyfunction!(simplify))?;
    m.add_class::<RobotConf>()?;
    m.add_class::<SpaceConf>()?;
    m.add_class::<RRTDubinsPlanner>()?;

    Ok(())
}
