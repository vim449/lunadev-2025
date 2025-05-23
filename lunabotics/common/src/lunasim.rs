use bitcode::{Decode, Encode};

#[derive(Debug, Encode, Decode, Clone)]
pub enum FromLunasim {
    Accelerometer {
        id: usize,
        acceleration: [f32; 3],
    },
    Gyroscope {
        id: usize,
        axis: [f32; 3],
        angle: f32,
    },
    DepthMap(Box<[u16]>),
    ExplicitApriltag {
        robot_axis: [f32; 3],
        robot_angle: f32,
        robot_origin: [f32; 3],
    },
}

#[derive(Debug, Encode, Decode, Clone)]
pub enum FromLunasimbot {
    PointCloud(Box<[[f32; 3]]>),
    Path(Box<[[f32; 3]]>),
    Thalassic {
        heightmap: Box<[f32]>,
        gradmap: Box<[f32]>,
        obstaclemap: Box<[bool]>,
    },
    Isometry {
        axis: [f32; 3],
        angle: f32,
        origin: [f32; 3],
    },
    Drive {
        left: f32,
        right: f32,
    },
    Quit,
}
