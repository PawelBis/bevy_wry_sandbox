use bevy::prelude::*;

#[derive(Debug)]
pub enum Error {
    EditorViewportCameraWithoutViewport,
}

#[derive(Resource, Default, Debug)]
pub struct EditorViewportUpdated {
    new_position: Option<UVec2>,
    new_size: Option<UVec2>,
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
    mut viewport_resized_event: ResMut<EditorViewportUpdated>,
    mut q_viewport_camera: Query<&mut Camera, With<EditorViewportCamera>>,
) -> Result<(), Error> {
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
        viewport.physical_position = new_position;
    }

    if let Some(new_size) = new_size {
        viewport.physical_size = new_size;
    }

    Ok(())
}
