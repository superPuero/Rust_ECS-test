use std::any::type_name;
use std::collections::HashMap;
use std::collections::HashSet;
use std::any::TypeId;
use std::any::Any;

struct Registry{
    counter: u32,
    entities: HashSet<u32>,
    component: HashMap<u32, HashMap<TypeId, Box<dyn Any>>>
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
        let c_key = TypeId::of::<C>();
        self.component
                    .entry(e_id)
                    .or_insert(HashMap::new())
                    .entry(c_key)
                    .or_insert(Box::new(comp));

        println!("Added component {:#?}, to entity {}", type_name::<C>(), e_id);
    }

    fn get_component<C: Any>(&mut self, e_id: u32) -> &mut C{
        self.component
                    .get_mut(&e_id)
                    .unwrap()
                    .get_mut(&TypeId::of::<C>())
                    .unwrap()
                    .downcast_mut::<C>()
                    .unwrap()
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
    r.add_component(id1, Foo{name: String::from("Fooer")});


    r.add_component(id2, Transform{x: 543, y: 345});
    r.add_component(id2, Foo{name: String::from("Boo")});


    let t1 = r.get_component::<Transform>(id1);
    println!("x: {}, y: {}", t1.x, t1.y);
    
    println!("name: {}", r.get_component::<Foo>(id1).name);

    
    let t2 = r.get_component::<Transform>(id2);
    println!("x: {}, y: {}", t2.x, t2.y);
    
    println!("name: {}", r.get_component::<Foo>(id2).name);

}
