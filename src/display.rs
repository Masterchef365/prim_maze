use anyhow::Result;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Matrix4, Object, Vertex, WinitBackend, UNLIT_FRAG, UNLIT_VERT,
};

pub type MeshData = (Vec<Vertex>, Vec<u16>);

struct DrawData {
    mesh: MeshData,
    animate: bool,
}

struct MyApp {
    maze: Object,
    n_maze_indices: usize,
    frame: usize,
    animate: bool,
}

impl App2D for MyApp {
    const TITLE: &'static str = "Visualizer";
    type Args = DrawData;

    fn new(engine: &mut WinitBackend, args: Self::Args) -> Result<Self> {
        let DrawData { mesh: (vertices, indices), animate } = args;

        let material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;

        let mesh = engine.add_mesh(&vertices, &indices)?;

        let margin = 0.10;
        let maze = Object {
            mesh,
            transform: Matrix4::new(
                2. - margin, 0., 0., margin - 1., //
                0., 2. - margin, 0., margin - 1., //
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
            animate,
            n_maze_indices,
        })
    }

    fn event(&mut self, _event: &WindowEvent, _engine: &mut WinitBackend) -> Result<()> {
        Ok(())
    }

    fn frame(&mut self, _engine: &mut WinitBackend) -> FramePacket {
        if self.animate {
            self.maze.subset = Some((self.n_maze_indices - self.frame % self.n_maze_indices) as u32);
            self.frame += self.n_maze_indices / (60 * 4);
        }
        FramePacket {
            objects: vec![self.maze],
        }
    }
}

pub fn visualize(mesh: MeshData, animate: bool) -> Result<()> {
    launch::<MyApp>(DrawData { mesh, animate })
}
