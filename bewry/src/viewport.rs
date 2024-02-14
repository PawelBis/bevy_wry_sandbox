use bevy::prelude::*;
use bevy_wry::ScaleFactor;

#[derive(Debug)]
pub enum Error {
    EditorViewportCameraWithoutViewport,
}

#[derive(Resource, Default, Debug)]
pub struct EditorViewportUpdated {
    pub new_position: Option<UVec2>,
    pub new_size: Option<UVec2>,
}

impl EditorViewportUpdated {
    fn try_consume(&mut self) -> Self {
        Self {
            new_position: self.new_position.take(),
            new_size: self.new_size.take(),
        }
    }
}

#[derive(Component)]
pub struct EditorViewportCamera;

/// Reads EditorViewportResized events and updates viewport
pub fn update_viewport(
    scale_factor: Res<ScaleFactor>,
    mut viewport_resized_event: ResMut<EditorViewportUpdated>,
    mut q_viewport_camera: Query<&mut Camera, With<EditorViewportCamera>>,
) -> Result<(), Error> {
    let scale_factor = scale_factor.as_f64();
    let mut camera = q_viewport_camera.single_mut();
    let viewport = camera
        .viewport
        .as_mut()
        .ok_or(Error::EditorViewportCameraWithoutViewport)?;

    let EditorViewportUpdated {
        new_position,
        new_size,
    } = viewport_resized_event.try_consume();
    if let Some(new_position) = new_position {
        let x = new_position.x as f64 * scale_factor;
        let y = new_position.y as f64 * scale_factor;
        let scaled_position = UVec2::new(x as u32, y as u32);
        viewport.physical_position = scaled_position;
    }

    if let Some(new_size) = new_size {
        let width = new_size.x as f64 * scale_factor;
        let height = new_size.y as f64 * scale_factor;
        let scaled_size = UVec2::new(width as u32, height as u32);
        viewport.physical_size = scaled_size;
    }

    Ok(())
}
