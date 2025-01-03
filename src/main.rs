use std::any::type_name;
use std::collections::HashMap;
use std::collections::HashSet;
use std::any::TypeId;
use std::any::Any;

struct Registry{
    counter: u32,
    entities: HashSet<u32>,
    component: HashMap<(u32, TypeId), Box<dyn Any>>
}

impl Registry{

    fn new() -> Self{

        println!("Creating registry");

        Registry{
            counter: 0,
            entities: HashSet::new(),
            component: HashMap::new(),
        }

    }
    fn add_entity(&mut self) -> u32{
        self.counter += 1;
        self.entities.insert(self.counter);

        println!("Added entity, ID: {}", self.counter);

        self.counter
    }

    fn add_component<C: Any>(&mut self, e_id: u32, comp: C){
        let c_id = TypeId::of::<C>();
        self.component
                    .entry((e_id, c_id))
                    .or_insert(Box::new(comp));

        println!("Added component {:#?}, to entity {}", type_name::<C>(), e_id);
    }

    fn get_component<C: Any>(&mut self, e_id: u32) -> Result<&mut C, String>{
        let key = &(e_id, TypeId::of::<C>());
        match self.component.get_mut(key){
            Some(val) => Ok(val.downcast_mut::<C>().unwrap()),
            None => Err(format!("No such element"))
        }     
    }

}

struct Transform{
    x: u32,
    y: u32
}
struct  Foo{
    name: String
}

fn main() {
    let mut r = Registry::new();
    let id1 = r.add_entity();
    let id2 = r.add_entity();

    r.add_component(id1, Transform{x: 12, y: 43});
    r.add_component(id1, Foo{name: "Fooer".into()});

    r.add_component(id2, Transform{x: 543, y: 345});
    r.add_component(id2, Foo{name: "boo".into()});

    let t1 = r.get_component::<Transform>(id1).unwrap();
    println!("x: {}, y: {}", t1.x, t1.y);
    
    println!("name: {}", r.get_component::<Foo>(id1).unwrap().name);
    
    let t2 = r.get_component::<Transform>(id2).unwrap();
    println!("x: {}, y: {}", t2.x, t2.y);
    
    println!("name: {}", r.get_component::<Foo>(id2).unwrap().name);
}
