pub mod view;

pub struct ListWareHouse {
    pub warehouses: Vec<WareHouse>,
}

#[derive(Clone)]
pub struct WareHouse {
    pub id_me: u64,
    pub active_tasks: u32,
    pub awaiting_fls: u32,
    pub forklifts: Vec<ForkLift>,
}

#[derive(Clone)]
pub struct ForkLift {
    pub id_me: u64,
    pub id_wh: u64,
    pub task: Option<u64>,
    pub state: FLState,
    pub point_l: PointData,
    pub speed: f32,
}

impl ForkLift {
    pub fn new(id_wh: u64, id: u64) -> Self {
        Self {
            id_me: id,
            id_wh,
            task: None,
            state: FLState::Awaiting,
            point_l: PointData::Detector(1),
            speed: 0.0,
        }
    }
}

#[derive(Clone)]
pub enum PointData {
    Detector(u32),
    Control(u32),
}

#[derive(Clone)]
pub enum FLState {
    Awaiting,
    GotTask,
    WayToItem,
    FoundItem,
    BackWItem,
    FinTask,
}

impl FLState {
    fn cycle(&mut self) -> Self {
        match *self {
            Self::Awaiting => Self::GotTask,
            Self::GotTask => Self::WayToItem,
            Self::WayToItem => Self::FoundItem,
            Self::FoundItem => Self::BackWItem,
            Self::BackWItem => Self::FinTask,
            Self::FinTask => Self::Awaiting,
        }
    }
}
