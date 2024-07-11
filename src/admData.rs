use std::rc::Rc;
use graphbench::editgraph::EditGraph;
use graphbench::graph::{Graph, Vertex, VertexMap, VertexSet};
use graphbench::iterators::EdgeIterable;

pub struct AdmData {
    id: Vertex,
    pub estimate: usize,
    pub n1_in_l: VertexSet,
    pub in_candidates: bool,
    pub m: VertexMap<Rc<AdmData>>, //key vertex v in M and in L, value neighbour of v in M and in R
    pub vias: VertexSet,
    pub set_vias: bool
}

impl AdmData {
    pub fn new(v: Vertex, v_neighbours:VertexSet) -> Self{
        AdmData {
            id: v,
            estimate: v_neighbours.len(),
            n1_in_l: v_neighbours,
            in_candidates: false,
            m:VertexMap::default(),
            vias: VertexSet::default(),
            set_vias: false
        }
    }

    //When a vertex is added to candidates we no longer need M
    pub fn delete_m(&mut self){
        self.m = VertexMap::default();
        self.in_candidates = true;
    }

    //When a vertex in N1(v) and in L is moving to R remove from L1 and see if it can be added to M
    pub fn remove_n1_in_l(&mut self, v:Rc<AdmData>)  {
        self.n1_in_l.remove(&v.id);
        self.estimate -= 1;

        for u in &v.n1_in_l {
            if !self.m.contains_key(&u) && !self.n1_in_l.contains(&u) && (u.clone() != self.id){
                self.m.insert(*u, Rc::clone(&v));
                self.estimate +=1;
                return;
            }
        }
    }

    //When a vertex in L and M is moving to R remove from M
    // also check to see if we can replace the edge being removed
    pub fn remove_from_m(&mut self, v:Rc<AdmData>){
        //remove v from m if v is in m
        let v_neighbour_in_m = self.m.remove(&v.id);

        match v_neighbour_in_m {
            None => return,
            Some(x) => {
                self.estimate -=1;
                //check if there is another vertex that can replace v
                for y in &x.n1_in_l {
                    if !self.m.contains_key(&y) && !self.n1_in_l.contains(&y) && (y.clone() != self.id) && (y.clone() != x.id){
                        self.m.insert(*y, Rc::clone(&x));
                        self.estimate +=1;
                        break;
                    }
                }
            }
        }
    }


    fn add_vias(&mut self, graph: &EditGraph, p:usize){
        let mut counter : VertexMap<usize> = VertexMap::default();
        let neighbours_v = graph.neighbours(&self.id);

        let _ = self.n1_in_l.iter().map(|x| self.vias.insert(*x));
        let _ = self.m.values().map(|x| self.vias.insert(x.id.clone()));

        //For each L in M count how many neighbours it has in R and M
        for (u,_) in &self.m {
            for w in self.m.values(){
                if w.n1_in_l.contains(&u){
                    *counter.entry(*u).or_default() += 1;
                }
            }
        }

        for w in neighbours_v{
            if !(self.m.contains_key(&w) && self.n1_in_l.contains(&w) ){
                for (u,_) in &self.m {
                    if graph.adjacent(u,w){
                        *counter.entry(*u).or_default() += 1;

                        if counter.get(u).unwrap() <= &(p + 1) {
                            self.vias.insert(*w);
                        }
                    }
                }
            }
        }
    }

    //TODO
    fn construct_g_for_augmenting_path(&self) {

    }

    //TODO
    fn augmenting_path(){
    }
}

#[cfg(test)]
mod test_adm_data {
    //TODO
}