
use crate::*;
use cgmath::{Matrix4, Vector3,Transform, Quaternion, Euler, Deg, Point3};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}



impl Vertex {
    pub fn new3dbox()->Vec<Vertex>{
        vec![

            // Front Face
            Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Back Face
            Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Right face
            Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Left Face
            Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Top Face
            Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Bottom Face
            Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
        ]
    }
}
pub struct Nscript3d {
    positions : HashMap<String,Vec<f32>>,
    rotations : HashMap<String,Vec<f32>>,
    scale : HashMap<String,Vec<f32>>,
    vertex : HashMap<String,Vec<Vertex>>,
    aabb : HashMap<String,AABB>,
    colliongroups : HashMap<String,Vec<String>>,
    rays: HashMap<String,Vec<(f32,f32,f32)>>
}
impl Nscript3d {
    pub fn new() -> Nscript3d{
        let this = Nscript3d{
            positions : HashMap::new(),
            rotations : HashMap::new(),
            scale : HashMap::new(),
            vertex : HashMap::new(),
            aabb : HashMap::new(),
            colliongroups : HashMap::new(),
            rays : HashMap::new(),
        };
        this
    }
    pub fn collisionbox_newbox(&mut self,objectname:&str) -> String{
        self.vertex.insert(objectname.to_string(),Vertex::new3dbox());
        self.positions.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.rotations.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.scale.insert(objectname.to_string(), vec![0.25,0.25,0.25]);
        objectname.to_string()
    }
    pub fn collisionbox_sizedbox(&mut self,objectname:&str,x:f32,y:f32,z:f32) -> String{
        self.vertex.insert(objectname.to_string(),Vertex::new3dbox());
        self.positions.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.rotations.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.scale.insert(objectname.to_string(), vec![x,y,z]);
        objectname.to_string()
    }
    pub fn collisionbox_addtogroup(&mut self, objectname:&str,group:&str){
        let mut getgroup = self.collisionbox_getgroup(group);
        getgroup.push(objectname.to_string());
        self.colliongroups.insert(group.to_string(), getgroup);
    }
    pub fn collisionbox_removefromgroup(&mut self,objectname:&str,group:&str){
        let mut getgroup = self.collisionbox_getgroup(group);
        getgroup.retain(|x| x != objectname);
        self.colliongroups.insert(group.to_string(), getgroup);
    }
    pub fn collisionbox_removegroup(&mut self,group:&str){
        self.colliongroups.insert(group.to_string(), Vec::new());
    }
    pub fn collisionbox_getgroup(&mut self,group:&str) -> Vec<String>{
        match self.colliongroups.get_key_value(group) {
            None => {
                Vec::new()
            }
                Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
    pub fn collisionbox_checkcollisions(&mut self,objectname:&str,group:&str) -> Vec<String>{
        let object_aabb = self.collisionbox_get_aabb(objectname);
        let mut collisionsvec:Vec<String> = Vec::new();
        for xunit in self.collisionbox_getgroup(group){
            let x_aabb = self.collisionbox_get_aabb(&xunit);
            if object_aabb.intersects(&x_aabb){
                if objectname!=xunit{
                    collisionsvec.push(xunit)
                }
            }
        }
        collisionsvec
    }

    fn getoffsets(&mut self ,objectname:&str)-> (Vec<f32> , Vec<f32>,Vec<f32>,Vec<Vertex>){
        //let getid = self.positions.get_key_value(objectname);
        let posvec = match self.positions.get_key_value(objectname) {
            None => {
                vec![0.0,0.0,0.0]
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        let rotvec = match self.rotations.get_key_value(objectname) {
            None => {
                vec![0.0,0.0,0.0]
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        let scalevec = match self.scale.get_key_value(objectname) {
            None => {
                vec![0.0,0.0,0.0]
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        let vertexvec = match self.vertex.get_key_value(objectname) {
            None => {
                Vertex::new3dbox()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        (posvec,rotvec,scalevec,vertexvec)

    }
    fn get_vertex(&mut self,objectname:&str) -> Vec<Vertex>{
        match self.vertex.get_key_value(objectname) {
            None => {
                Vertex::new3dbox()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
    fn collisionbox_get_aabb(&mut self,objectname:&str) -> AABB{
        match self.aabb.get_key_value(objectname) {
            None => {
                let vertex = self.get_vertex(&objectname);
                let aabb = AABB::from_vertices(&vertex);
                self.aabb.insert(objectname.to_string(), aabb.clone());
                    aabb
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
    pub fn collisionbox_setposition(&mut self, objectname:&str,posx: f32,posy:f32,posz:f32){
        let newpos = vec![posx,posy,posz];
        self.positions.insert(objectname.to_string(), newpos);
        self.updatevertex(objectname);
    }
    pub fn collisionbox_setrotation(&mut self, objectname:&str,posx: f32,posy:f32,posz:f32){
        let newpos = vec![posx,posy,posz];
        self.rotations.insert(objectname.to_string(), newpos);
        self.updatevertex(objectname);
    }
    pub fn collisionbox_setscale(&mut self, objectname:&str,posx: f32,posy:f32,posz:f32){
        let newpos = vec![posx,posy,posz];
        self.scale.insert(objectname.to_string(), newpos);
        self.updatevertex(objectname);
    }

    fn updatevertex(&mut self,objectname:&str){
        let (pos,rot,scale,_) = self.getoffsets(&objectname);
        let newvertex = Nscript3d::calculate_transformed_vertices(Vertex::new3dbox(),pos,rot,scale);
        self.aabb.insert(objectname.to_string(), AABB::from_vertices(&newvertex));
        self.vertex.insert(objectname.to_string(), newvertex);
    }
    fn calculate_transformed_vertices(
        vertices: Vec<Vertex>,
        position: Vec<f32>,
        rotation: Vec<f32>, // [pitch, yaw, roll]
        scale: Vec<f32>,
    ) -> Vec<Vertex> {
        let position_vec = Vector3::new(position[0], position[1], position[2]);
        let scale_vec = Vector3::new(scale[0], scale[1], scale[2]);

        // Convert Euler angles from degrees to radians and create a quaternion
        let euler_rotation = Euler {
            x: Deg(rotation[0]), // pitch (X)
            y: Deg(rotation[1]), // yaw (Y)
            z: Deg(rotation[2]), // roll (Z)
        };
        let rotation_quaternion: Quaternion<f32> = Quaternion::from(euler_rotation);

        // Create transformation matrices
        let translation_matrix = Matrix4::from_translation(position_vec);
        let rotation_matrix = Matrix4::from(rotation_quaternion);
        let scale_matrix = Matrix4::from_nonuniform_scale(scale_vec.x, scale_vec.y, scale_vec.z);

        // Combine into a single transformation matrix in the correct order: Scale -> Rotate -> Translate
        let transformation_matrix = translation_matrix * rotation_matrix * scale_matrix;

        // Transform all vertices by applying the transformation matrix
        let transformed_vertices: Vec<Vertex> = vertices
            .into_iter()
            .map(|vertex| {
                let position_vec = Point3::new(vertex.position[0], vertex.position[1], vertex.position[2]);
                let transformed_position = transformation_matrix.transform_point(position_vec);

                Vertex {
                    position: [transformed_position.x, transformed_position.y, transformed_position.z],
                    uv: vertex.uv,
                    normal: vertex.normal,
                }
            })
            .collect();

        // Return the transformed vertices and the same indices
        transformed_vertices
    }
    pub fn castray(&mut self,rayid:&str,start: (f32, f32, f32), target: (f32, f32, f32), step: f32)-> usize{
        let (x0, y0, z0) = start;
        let (x1, y1, z1) = target;

        let dx = x1 - x0;
        let dy = y1 - y0;
        let dz = z1 - z0;

        let distance = ((dx * dx) + (dy * dy) + (dz * dz)).sqrt();
        let steps = (distance / step).ceil() as usize;
        let mut points = Vec::with_capacity(steps + 1);

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = x0 + t * dx;
            let y = y0 + t * dy;
            let z = z0 + t * dz;
            points.push((x, y, z).into());
        }
        let lenght = points.len();
        self.rays.insert(rayid.to_string(),points);
        lenght
    }
    pub fn getraypoint(&mut self,rayid:&str,entree:usize)-> (f32,f32,f32){
        if let Some(ray) = self.rays.get(rayid.into()){
            if ray.len() > entree{
                return ray[entree].clone();
            }
        }

        (0.0,0.0,0.0)
    }

    pub fn removeray(&mut self,rayid:&str){
       self.rays.remove(rayid.into());
    }
}

#[derive(Debug, Clone)]
struct AABB {
    min: [f32; 3],
    max: [f32; 3],
}

impl AABB {
    // Create AABB from a list of vertices
    pub fn from_vertices(vertices: &[Vertex]) -> Self {
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        for vertex in vertices {
            for i in 0..3 {
                if vertex.position[i] < min[i] {
                    min[i] = vertex.position[i];
                }
                if vertex.position[i] > max[i] {
                    max[i] = vertex.position[i];
                }
            }
        }

        AABB { min, max }
    }

    // Check if two AABBs intersect
    pub fn intersects(&self, other: &AABB) -> bool {
        for i in 0..3 {
            if self.max[i] < other.min[i] || self.min[i] > other.max[i] {
                return false;
            }
        }
        true
    }
}

pub fn nscriptfn_objectgetrangebetween(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {

    let mut var = NscriptVar::new("ray");
    let obj1 = storage.getargstring(args[0], block);
    let obj2 = storage.getargstring(args[1], block);
    let vector1 = vec!(Nstring::f32(&storage.objectgetprop(&obj1,"x").stringdata),Nstring::f32(&storage.objectgetprop(&obj1,"y").stringdata),Nstring::f32(&storage.objectgetprop(&obj1,"z").stringdata));
    let vector2 = vec!(Nstring::f32(&storage.objectgetprop(&obj2,"x").stringdata),Nstring::f32(&storage.objectgetprop(&obj2,"y").stringdata),Nstring::f32(&storage.objectgetprop(&obj2,"z").stringdata));
    let mut sum_squares_diff = 0.0;
    for (x1, x2) in vector1.iter().zip(vector2.iter()) {
        sum_squares_diff += (x1 - x2).powi(2);
    }
    var.stringdata = sum_squares_diff.sqrt().to_string();
    return var;
}


pub fn nscriptfn_castray(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("ray");
    let rayid = storage.getargstring(args[0], block);
    let pos_a = storage.getargstringvec(&args[1],block);
    let pos1 = (Nstring::f32(&pos_a[0]),Nstring::f32(&pos_a[1]),Nstring::f32(&pos_a[2]));

    let pos_b = storage.getargstringvec(&args[2],block);
    let pos2 = (Nstring::f32(&pos_b[0]),Nstring::f32(&pos_b[1]),Nstring::f32(&pos_b[2]));
    let step = Nstring::f32(&storage.getargstring(args[3], block));
    var.stringdata = storage.nscript3d.castray(&rayid,pos1,pos2,step).to_string();
    return var;
}
pub fn nscriptfn_getraypoint(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("ray");
    let rayid = storage.getargstring(args[0], block);
    let entree = Nstring::usize(&storage.getargstring(args[1], block));
    let point = storage.nscript3d.getraypoint(&rayid,entree);
    var.stringvec = vec!(point.0.to_string(),point.1.to_string(),point.2.to_string());
    return var;
}
pub fn nscriptfn_aabb_newbox(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("addbox");
    let objname = storage.getargstring(&args[0],block);
    var.stringdata = storage.nscript3d.collisionbox_newbox(&objname);
    return var;
}
pub fn nscriptfn_aabb_sizedbox(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("addbox");
    if args.len() > 3 {
        let objname = storage.getargstring(&args[0],block);
        let x = storage.getargstring(&args[1],block);
        let y = storage.getargstring(&args[2],block);
        let z = storage.getargstring(&args[3],block);
        var.stringdata = storage.nscript3d.collisionbox_sizedbox(&objname,Nstring::f32(&x),Nstring::f32(&y),Nstring::f32(&z));
    }
    return var;
}
pub fn nscriptfn_aabb_addtogroup(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let var = NscriptVar::new("aabb");
    if args.len() > 1 {
        let objname = storage.getargstring(&args[0],block);
        let group = storage.getargstring(&args[1],block);
        storage.nscript3d.collisionbox_addtogroup(&objname,&group);
    }
    return var;
}
pub fn nscriptfn_aabb_removefromgroup(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let var = NscriptVar::new("aabb");
    if args.len() > 1 {
        let objname = storage.getargstring(&args[0],block);
        let group = storage.getargstring(&args[1],block);
        storage.nscript3d.collisionbox_removefromgroup(&objname,&group);
    }
    return var;
}
pub fn nscriptfn_aabb_getgroup(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("aabb");
    let object = storage.getargstring(&args[0],block);
    var.stringvec = storage.nscript3d.collisionbox_getgroup(&object);
    return var;
}
pub fn nscriptfn_aabb_removegroup(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let var = NscriptVar::new("aabb");
    let object = storage.getargstring(&args[0],block);
    storage.nscript3d.collisionbox_removegroup(&object);
    return var;
}
pub fn nscriptfn_aabb_getcollisions(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("aabb");
    if args.len() > 1 {
        let object = storage.getargstring(&args[0],block);
        let group = storage.getargstring(&args[1],block);
        var.stringvec = storage.nscript3d.collisionbox_checkcollisions(&object,&group);
    }

    return var;
}
pub fn nscriptfn_aabb_setposition(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let var = NscriptVar::new("addbox");
    if args.len() > 3 {
        let objname = storage.getargstring(&args[0],block);
        let x = storage.getargstring(&args[1],block);
        let y = storage.getargstring(&args[2],block);
        let z = storage.getargstring(&args[3],block);
        storage.nscript3d.collisionbox_setposition(&objname,Nstring::f32(&x),Nstring::f32(&y),Nstring::f32(&z));
    }
    return var;
}
pub fn nscriptfn_aabb_setscale(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let var = NscriptVar::new("addbox");
    if args.len() > 3 {
        let objname = storage.getargstring(&args[0],block);
        let x = storage.getargstring(&args[1],block);
        let y = storage.getargstring(&args[2],block);
        let z = storage.getargstring(&args[3],block);
        storage.nscript3d.collisionbox_setscale(&objname,Nstring::f32(&x),Nstring::f32(&y),Nstring::f32(&z));
    }
    return var;
}
pub fn nscriptfn_aabb_setrotation(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let var = NscriptVar::new("addbox");
    if args.len() > 3 {
        let objname = storage.getargstring(&args[0],block);
        let x = storage.getargstring(&args[1],block);
        let y = storage.getargstring(&args[2],block);
        let z = storage.getargstring(&args[3],block);
        storage.nscript3d.collisionbox_setrotation(&objname,Nstring::f32(&x),Nstring::f32(&y),Nstring::f32(&z));
    }
    return var;
}
