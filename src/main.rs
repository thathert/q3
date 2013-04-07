/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: main.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Entry point.
*/

extern mod std;
extern mod opengles;
extern mod glfw;

#[path = "gl/gl.rs"]
mod gl;

#[path = "math/math.rs"]
mod math;

#[macro_escape]
#[path = "gl/check.rs"]
mod check;

#[path = "obj/bsp/map.rs"]
mod map;

fn main() {
  glfw::set_error_callback(error_callback);

  do glfw::spawn {
    let window = @glfw::Window::create(1024, 768, "Q^3", glfw::Windowed).unwrap();
    window.make_context_current();

    let camera = @mut gl::Camera::new(window);
    camera.init();

    /* Setup callbacks. */
    window.set_input_mode(glfw::CURSOR_MODE, glfw::CURSOR_CAPTURED as int);
    do window.set_size_callback |_, width, height|
    { camera.resize(width as i32, height as i32); }
    do window.set_cursor_pos_callback |_, x, y|
    { camera.mouse_moved(x as i32, y as i32); }
    do window.set_key_callback |window, key, action|
    {
      camera.key_action(key, action);
      key_callback(window, key, action);
    }

    let map = map::Map::new("data/q3ctf1.bsp");
    //let map = map::Map::new("data/map.bsp");
    //let map = map::Map::new("data/dk.bsp");

    /* Shader Creation. */
    let shader_vert_src =
                          ~"#version 330
                            uniform mat4x4 proj;
                            uniform mat4x4 world;
                            layout (location = 0) in vec3 in_position;
                            layout (location = 1) in vec4 in_color;
                            out vec4 trans_color;
                            void main() {
                              gl_Position = proj * world * vec4(in_position.x, in_position.y, in_position.z, 1.0); 
                              trans_color = in_color;
                            }";
    let shader_frag_src =
                          ~"#version 330
                            in vec4 trans_color;
                            out vec4 out_color;
                            void main() {
                              out_color = vec4(1.0, 1.0, 1.0, 1.0);
                              //out_color = vec4( trans_color.y, 
                              //                  5.0 - trans_color.z, 
                              //                  0.5 - out_position.y, 
                              //                  1.0);
                              //out_color = vec4(trans_color, 1.0);
                              out_color = trans_color;
                              if(out_color.r > 1.0 && out_color.g > 1.0 && out_color.b > 1.0)
                                out_color /= 255.0;
                              out_color.a = 1.0;
                            }";
    let shader = gl::Shader::new(shader_vert_src, shader_frag_src);
    shader.bind();

    let proj_loc = shader.get_uniform_location(~"proj");
    let world_loc = shader.get_uniform_location(~"world");

    let mut cur_time = (std::time::precise_time_ns() / 100000) as f32; // Tenth of a second
    let mut last_time = cur_time;

    while !window.should_close() {
      glfw::poll_events();

      let delta = cur_time - last_time;
      last_time = cur_time;
      cur_time = (std::time::precise_time_ns() / 100000) as f32;

      camera.update(delta);
      shader.update_uniform(proj_loc, camera.projection);
      shader.update_uniform(world_loc, camera.view);

      check!(gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
      {
        map.draw();
      } window.swap_buffers();

      std::timer::sleep(@std::uv::global_loop::get(), 16);
    }
  }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int)
{
  if action == glfw::PRESS && key == glfw::KEY_ESCAPE
  { window.set_should_close(true); }
}

fn error_callback(error: libc::c_int, description: ~str)
{ io::println(fmt!("GLFW Error %?: %s", error, description)); }

