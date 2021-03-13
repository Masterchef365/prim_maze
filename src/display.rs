use anyhow::Result;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Matrix4, Object, Vertex, WinitBackend, UNLIT_FRAG, UNLIT_VERT,
};

struct MyApp {
    maze: Object,
    n_maze_indices: usize,
    frame: usize,
}

pub type MeshData = (Vec<Vertex>, Vec<u16>);

impl App2D for MyApp {
    const TITLE: &'static str = "Visualizer";
    type Args = MeshData;

    fn new(engine: &mut WinitBackend, (vertices, indices): Self::Args) -> Result<Self> {
        let material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;

        let mesh = engine.add_mesh(&vertices, &indices)?;

        let maze = Object {
            mesh,
            /*
            transform: Matrix4::new(
                2., 0., 0., -1., //
                0., 2., 0., -1., //
                0., 0., 1., 0., //
                0., 0., 0., 1., //
            ),
            */
            transform: Matrix4::new(
                2., 0., 0., -1., //
                0., 2., 0., -1., //
                0., 0., 1., 0., //
                0., 0., 0., 1., //
            ),
            material,
            subset: None,
        };

        let n_maze_indices = indices.len();
        Ok(Self {
            maze,
            frame: 0,
            n_maze_indices,
        })
    }

    fn event(&mut self, _event: &WindowEvent, _engine: &mut WinitBackend) -> Result<()> {
        Ok(())
    }

    fn frame(&mut self, _engine: &mut WinitBackend) -> FramePacket {
        self.maze.subset = Some((self.frame % self.n_maze_indices) as u32);
        self.frame += 1;
        FramePacket {
            objects: vec![self.maze],
        }
    }
}

pub fn visualize(mesh: MeshData) -> Result<()> {
    launch::<MyApp>(mesh)
}
