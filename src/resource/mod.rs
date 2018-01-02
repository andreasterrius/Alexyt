use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Cursor};
use std::{self, fmt};
use image;

pub struct ResourceManager {
    images : HashMap<String, ResourceImageFile>,
    glsl : HashMap<String, ResourceGlslFile>,
    audios : HashMap<String, ResourceAudioFile>
}

impl ResourceManager {

    pub fn new() -> ResourceManager {
        ResourceManager{
            images: HashMap::new(),
            glsl: HashMap::new(),
            audios : HashMap::new()
        }
    }

    pub fn load_image(&mut self, key : &str, path : &str){

        let img_path = PathBuf::from(path);
        let img = image::open(path)
            .expect(&format!("Image not found, {:?}", path));

        self.images.insert(String::from(key), ResourceImageFile{
            image : img
        });
    }

    pub fn get_image(&self, key: &str) -> Option<&ResourceImageFile> {
        self.images.get(key)
    }

    pub fn load_glsl(&mut self,
                 key: &str,
                 vertex_shader_path: &str,
                 fragment_shader_path: &str)
    {
        let vertex_shader_pathbuf = PathBuf::from(vertex_shader_path);
        let mut vertex_shader = String::new();
        File::open(vertex_shader_pathbuf)
            .expect(&format!("Vertex shader file not found, {:?}", vertex_shader_path))
            .read_to_string(&mut vertex_shader);

        let fragment_shader_pathbuf = PathBuf::from(fragment_shader_path);
        let mut fragment_shader = String::new();
        File::open(fragment_shader_pathbuf)
            .expect(&format!("Fragment shader file not found. {:?}", fragment_shader_path))
            .read_to_string(&mut fragment_shader);

        self.glsl.insert(String::from(key), ResourceGlslFile{
            vertex_shader,
            fragment_shader,
        });
    }

    pub fn get_glsl(&self, key : &str) -> Option<&ResourceGlslFile> {
        self.glsl.get(key)
    }

    pub fn load_audio(&mut self, key : &str, path : &str) {
        let audio_path = PathBuf::from(path);
        let audio_file = File::open(path)
            .expect(&format!("Audio file not found, {:?}", path));
        let audio_data = ResourceManager::load_binary_file(audio_file);

        self.audios.insert(String::from(key), ResourceAudioFile{ audio : audio_data } );
    }

    pub fn get_audio(&self, key : &str) -> Option<&ResourceAudioFile> {
        self.audios.get(key)
    }

    fn load_binary_file( file : File ) -> Vec<u8> {
        let mut vec = vec!();
        let _ = BufReader::new(file).read_to_end(&mut vec);
        vec
    }
}

#[derive(Debug)]
pub struct ResourceGlslFile {
    pub vertex_shader : String,
    pub fragment_shader : String
}

pub struct ResourceImageFile {
    pub image : image::DynamicImage
}

#[derive(Debug)]
pub struct ResourceAudioFile {
    pub audio : Vec<u8>
}
