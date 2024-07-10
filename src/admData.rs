use std::rc::Rc;
use graphbench::graph::{Vertex, VertexMap, VertexSet};

pub struct AdmData {
    id: Vertex,
    pub estimate: usize,
    pub l1: VertexSet,
    pub m: VertexMap<Rc<AdmData>>, //key vertex v in M and L2, value neighbour of v in M and R
    pub vias: VertexMap<Rc<AdmData>>,
    pub set_vias: bool
}

impl AdmData {
    pub fn new(id: Vertex, l1:VertexSet) -> Self{
        AdmData {
            id,
            estimate: l1.len(),
            l1,
            m:VertexMap::default(),
            vias: VertexMap::default(),
            set_vias: false
        }
    }

    //Adds a vertex v moving from L1 to R, to M
    //if v has a neighbour not in M or in L1
    fn add_to_m(&mut self, v: &Rc<AdmData>){
        for u in &v.l1{
            if !self.m.contains_key(&u) && !self.l1.contains(&u) && (u.clone() != self.id){
                self.m.insert(*u, Rc::clone(&v));
                self.estimate +=1;
                return;
            }
        }
    }

    //When a vertex in L1 is moving to R remove from L1 and see if it can be added to M
    pub fn remove_l1(&mut self, p:usize, v:Rc<AdmData>)  {
        self.l1.remove(&v.id);
        self.estimate -= 1;
        self.add_to_m(&v);
        if self.set_vias && self.m.len()==p {
            self.vias.insert(v.id, v);
        }
    }

    //When a vertex v in L2 and M is moving to R remove from M
    // also check to see if we can replace the edge being removed
    pub fn remove_from_m(&mut self, v:Rc<AdmData>){
        //remove v from m if v is in m
        let v_neighbour_in_m = self.m.remove(&v.id);

        match v_neighbour_in_m {
            None => return,
            Some(u) => {
                self.estimate -=1;
                //check if there is another vertex that can replace v
                for w in &u.l1{
                    if !self.m.contains_key(&w) && !self.l1.contains(&w) && (w.clone() != self.id) && (w.clone() != u.id){
                        self.m.insert(*w, Rc::clone(&u));
                        self.estimate +=1;
                        break;
                    }
                }
            }
        }
    }

    fn add_vias(&mut self, v_r1_neighbours: Vec<Rc<AdmData>>, p:usize){
        let mut counter : VertexMap<usize> = VertexMap::default();
        for u in v_r1_neighbours{
            for (w,_) in &self.m {
                if u.l1.contains(w){
                    *counter.entry(*w).or_default() += 1;
                    let num_vias_for_w = counter.get(w).unwrap();
                    if num_vias_for_w <= &p {
                        self.vias.insert(u.id.clone(), Rc::clone(&u));
                    }
                }
            }
        }
        self.set_vias = true;
        return;
    }

    fn construct_g_for_augmenting_path(&self) -> Vec<(Vertex, Vertex)> {
        //TODO Might be better using hash map instead???
        let mut edges : Vec<(Vertex,Vertex)> = Vec::default();
        let mut vertices_in_m_and_r = Vec::default();

        //Iterate through all vertices v in M and R1
        //and add edges between vertices in M
        //If v has neighbours in L1 not in M include one of them
        for (_, v) in &self.m {
            let mut edge_to_outside_m = false;
            vertices_in_m_and_r.push(v.id);
            for u in &v.l1 {
                if self.m.contains_key(&u){
                    edges.push((v.id, *u));
                }else if !edge_to_outside_m {
                    edges.push((v.id, *u));
                    //also add edge from root to vertex in M and R and has neighbours in L1 not in M
                    edges.push((v.id, self.id));
                    edge_to_outside_m = true;
                }
            }
        }

        //Get edges between vertices in R that is not in M and vertices in M
        for (v, v_adm_data) in &self.vias{
            if !vertices_in_m_and_r.contains(v){
                for u in &v_adm_data.l1{
                    if self.m.contains_key(&u){
                        edges.push((*v, *u));
                    }
                }
            }
        }

        return edges;
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