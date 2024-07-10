use std::rc::Rc;
use graphbench::graph::{Vertex, VertexMap, VertexSet};

pub struct AdmData {
    id: Vertex,
    pub estimate: usize,
    pub l1: VertexSet,
    pub m2: VertexMap<Rc<AdmData>>,
    pub vias: VertexSet,
    pub set_vias: bool
}

impl AdmData {
    pub fn new(id: Vertex, l1:VertexSet) -> Self{
        AdmData {
            id,
            estimate: l1.len(),
            l1,
            m2:VertexMap::default(),
            vias: VertexSet::default(),
            set_vias: false
        }
    }

    fn add_to_m(&mut self, v: &Rc<AdmData>){
        for u in &v.l1{
            //Can only add to M if v's L1 neighbours is not in M or in L1
            if !self.m2.contains_key(&u) && !self.l1.contains(&u) && (u.clone() != self.id){
                self.m2.insert(*u, Rc::clone(&v));
                self.estimate +=1;
                return;
            }
        }
    }

    pub fn remove_l1(&mut self, p:usize, v:Rc<AdmData>)  {
        self.l1.remove(&v.id);
        self.estimate -= 1;
        self.add_to_m(&v);
        if self.set_vias && self.m2.len()==p {
            self.vias.insert(v.id);
        }
    }

    pub fn remove_from_m(&mut self, v:Rc<AdmData>){
        //remove v from m if v is in m
        let v_neighbour_in_m = self.m2.remove(&v.id);

        match v_neighbour_in_m {
            None => return,
            Some(u) => {
                self.estimate -=1;
                //check if there is another vertex that can replace v
                for w in &u.l1{
                    if !self.m2.contains_key(&w) && !self.l1.contains(&w) && (w.clone() != self.id) && (w.clone() != u.id){
                        self.m2.insert(*w, Rc::clone(&u));
                        self.estimate +=1;
                        break;
                    }
                }
            }
        }
    }

    fn add_vias(&mut self, v_r1_neighbours: Vec<AdmData>, p:usize){
        let mut counter : VertexMap<usize> = VertexMap::default();
        for u in v_r1_neighbours{
            for (w,_) in &self.m2{
                if u.l1.contains(w){
                    *counter.entry(*w).or_default() += 1;
                    let num_vias_for_w = counter.get(w).unwrap();
                    if num_vias_for_w <= &p {
                        self.vias.insert(u.id);
                    }
                }
            }
        }
        self.set_vias = true;
        return;
    }

    //TODO
    fn construct_g_for_augmenting_path(&self){
        let mut edges : Vec<(Vertex,Vertex)> = Vec::default();

        //Iterate through all the M1's
        for (_, v) in &self.m2{

        }
    }

    //TODO
    fn augmenting_path(){
        //TODO if no edges between M and vertex in L2 not in M
        //and between M and vertex in R1 not in M
        //do not do augmenting path
    }
}

#[cfg(test)]
mod test_adm_data {
    //TODO
}