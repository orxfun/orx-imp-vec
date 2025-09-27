use orx_imp_vec::*;

#[derive(Default)]
struct Bag {
    things: ImpVec<Thing>,
}

impl Bag {
    fn push_thing(&self, name: &str) -> ThingInBag<'_> {
        self.things.imp_push(Thing {
            name: name.to_string(),
        });
        ThingInBag {
            bag: self,
            thing: &self.things[self.things.len() - 1],
        }
    }

    fn things(&self) -> Vec<&str> {
        self.things.iter().map(|x| x.name.as_str()).collect()
    }
}

struct Thing {
    name: String,
}

#[derive(Clone, Copy)]
struct ThingInBag<'a> {
    thing: &'a Thing,
    bag: &'a Bag,
}
impl<'a> ThingInBag<'a> {
    fn push_thing_in_same_bag(&self, name: &str) -> ThingInBag<'a> {
        self.bag.push_thing(name)
    }

    fn is_in_bag(&self, bag: &Bag) -> bool {
        let self_bag = self.bag as *const Bag;
        let other_bag = bag as *const Bag;
        self_bag == other_bag
    }
}
impl<'a> PartialEq for ThingInBag<'a> {
    fn eq(&self, other: &Self) -> bool {
        let same_bag = self.bag as *const Bag == other.bag as *const Bag;
        let same_thing = self.thing as *const Thing == other.thing as *const Thing;
        same_bag && same_thing
    }
}

fn main() {
    // create a bag
    let bag = Bag::default();

    // add things and collect things in bag
    let pen = bag.push_thing("pen");
    let cup = bag.push_thing("cup");
    assert_eq!(bag.things(), ["pen", "cup"]);

    // add new things to bag using existing things in bag
    pen.push_thing_in_same_bag("pencil");
    cup.push_thing_in_same_bag("cupcake");
    assert_eq!(bag.things(), ["pen", "cup", "pencil", "cupcake"]);

    // create another bag
    let other_bag = Bag::default();
    let key = other_bag.push_thing("key");
    let other_pen = other_bag.push_thing("pen");

    // check if things belong to the same bag in constant time
    assert_eq!(pen.is_in_bag(&bag), true);
    assert_eq!(pen.is_in_bag(&other_bag), false);

    assert_eq!(key.is_in_bag(&bag), false);
    assert_eq!(key.is_in_bag(&other_bag), true);

    // use referential equality to compare if two things are the same
    assert!(pen != other_pen);
}
