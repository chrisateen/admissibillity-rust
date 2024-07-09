use std::collections::{HashMap};
use std::rc::Rc;
use graphbench::graph::{Vertex, VertexMap, VertexSet};

pub struct M1 {
    pub matching_partner: Vertex,
}

pub struct M2 {
    pub matching_partner: Vertex,
    pub unmatched_r1_neighbours: VertexSet,
}

pub struct AdmData {
    id: Vertex,
    pub estimate: usize,
    pub l1: HashMap<Vertex,Rc<AdmData>>,
    pub r1: HashMap<Vertex,Rc<AdmData>>,
    pub m1: VertexMap<M1>,
    pub m2: VertexMap<M2>,
}

impl AdmData {
    pub fn new(id: Vertex, l1:Vec<AdmData>) -> Self{
        let estimate = l1.len();
        let mut add_l1: HashMap<Vertex, Rc<AdmData>> = HashMap::default();
        for adm_data in l1 {
            add_l1.insert(adm_data.id, Rc::new(adm_data));
        }
       AdmData {
           id,
           estimate,
           l1: add_l1,
           r1: HashMap::default(),
           m1: VertexMap::default(),
           m2: VertexMap::default()
       }
    }

    //Gets all of v neighbours that is not already in M2
    fn get_eligible_m2(&self, l2_via_v:&VertexSet) -> VertexSet {
        let mut eligible_m2 : VertexSet = VertexSet::default();
        for v in l2_via_v {
            if !self.m2.contains_key(&v){
                eligible_m2.insert(*v);
            }
        }
        return eligible_m2;
    }

    //When removing a vertex in M2 there might be another vertex that can replace it
    fn get_replaceable_m2_vertex(&self, v:&Vertex) -> Option<Rc<AdmData>> {
        let u = self.m2.get(v).unwrap().matching_partner;
        let u_l1 = &self.r1.get(&u).unwrap().l1;

        //Find any neighbours of u that's not in L1 of v or in M2
        for (w, adm_data_v) in u_l1{
            if !self.l1.contains_key(w) && !self.m2.contains_key(w) && w != v{
                return Some(adm_data_v.clone())
            }
        }
        return None;
    }

    pub fn remove_vertex_from_l1(&mut self, v:&Vertex){
        self.l1.remove(v);
        self.estimate -=1;
    }

    pub fn add_vertices_to_m(&mut self, v:Rc<AdmData>, v_neighbours:&VertexSet, p: usize){
        let mut l2_via_v: VertexSet = VertexSet::default();

        for u in v_neighbours {
            if !self.l1.contains_key(u){
                l2_via_v.insert(*u);
            }
        }

        //If all of v neighbours is in L1 already do nothing
        if l2_via_v.len() == 0 {
            return;
        }

        self.r1.insert(v.id, Rc::clone(&v));

        let eligible_m2  = self.get_eligible_m2(&l2_via_v);

        //If all of v neighbours is already in M2
        //for each of v's neighbours add v as R1 unmatched neighbours
        if eligible_m2.len() == 0 {
            for u in &l2_via_v {
                let m2 = self.m2.get_mut(&u).unwrap();
                let unmatched_neighbours = &mut m2.unmatched_r1_neighbours;
                // for each M2 we only store p + 1 unmatched neighbours in r1
                if unmatched_neighbours.len() <= p {
                    unmatched_neighbours.insert(*u);
                }
            }
            return;
        }

        //Add a neighbour of v not in M2 into M
        let v_matching_partner = eligible_m2.iter().next().unwrap();
        let new_m1 = M1{ matching_partner: *v_matching_partner };
        let new_m2 = M2{ matching_partner: v.id, unmatched_r1_neighbours: VertexSet::default()};
        self.m1.insert(v.id, new_m1);
        self.m2.insert(*v_matching_partner, new_m2);
        self.estimate +=1;
    }

    //Remove an M2 vertex that is now moving into R
    pub fn remove_m2(&mut self, v:&Vertex, p:usize){
        let v_matching_partner = self.m2.get(v).unwrap().matching_partner;
        let v_replacement = self.get_replaceable_m2_vertex(v);

        match v_replacement {
            //If v can't be replaced with another vertex remove v, and it's partner from m
            None => {
                self.m2.remove(v);
                self.m1.remove(&v_matching_partner);
                self.estimate -= 1;
                return;
            }
            Some(_)=> {
                let u = v_replacement.unwrap();
                let new_m1 = M1{ matching_partner: u.id };
                let mut new_m2 = M2{ matching_partner: v_matching_partner, unmatched_r1_neighbours: VertexSet::default()};
                //Check if there is any r1 in v_replacement that is not in m1 to add into unmatched
                for (w, _) in &u.r1{
                    //only store p + 1 unmatched neighbours in r1 for a vertex in m2
                    if new_m2.unmatched_r1_neighbours.len() == p + 1{
                        break;
                    }
                    if !self.m1.contains_key(&w){
                        new_m2.unmatched_r1_neighbours.insert(*w);
                    }
                }
                self.m2.remove(v);
                self.m2.insert(u.id, new_m2);
                self.m1.insert(v_matching_partner, new_m1);
            }
        }
    }

    //TODO is this the best way to prepare M for augmented path
    //TODO Don't bother with matching if both M1 & M2 do not have at least 1 vertex with unmatched neighbour
    fn prepare_for_augmenting_path(&self) -> Vec<(&Vertex, &Vertex)> {
        let mut edges = Vec::default();

        //Gets all the edges from M1
        for (v, m1) in &self.m1 {
            let v_matching_partner = &m1.matching_partner;
            let v_adm = self.r1.get(&v).unwrap();
            let mut v_has_unmatched_partner = false;

            for (w, _) in &v_adm.l1{
                //Add edges between M1 and M2
                if self.m2.contains_key(&w){
                    edges.push((v, w));
                }
                //Add a single edge between an M1 and a L2 not in M2
                if !v_has_unmatched_partner{
                    edges.push((v, w));
                    v_has_unmatched_partner = true;
                }
            }
            edges.push((v, v_matching_partner));

        }

        //Add edges from M2 to an unmatched partner
        for (v, m2) in &self.m2 {
            if m2.unmatched_r1_neighbours.len() > 0{
                edges.push((&v, &*m2.unmatched_r1_neighbours.iter().next().unwrap()));
            }
        }

        return edges;
    }

    //TODO Update M if size of matching is p + 1
    fn update_m (&self){
        return;
    }

    //TODO
    pub fn should_add_to_candidates(&self, p:usize) -> bool {
        //if estimate > p no need to do augmenting path
        if self.estimate > p {
            return false;
        }

        let matching = self.prepare_for_augmenting_path();

        return true;
    }


}

#[cfg(test)]
mod test_adm_data {
    use std::rc::Rc;
    use graphbench::graph::{ VertexSet };
    use crate::admData2::{AdmData, M1, M2};

    #[test]
    fn add_vertices_should_not_add_v_to_r1_if_neighbours_of_v_is_in_l1(){
        let v = AdmData::new(2, Vec::new());
        let l1: Vec<AdmData>= vec![
            AdmData::new(2, Vec::new()),
            AdmData::new(3, Vec::new())
        ];
        let mut u = AdmData::new(1,  l1);
        u.add_vertices_to_m(Rc::new(v), &vec![2, 3].iter().cloned().collect(), 1);
        assert_eq!(u.r1.len(), 0);
        assert!(!u.r1.contains_key(&2));
    }

    #[test]
    fn add_vertices_should_update_unmatched_if_v_neighbours_is_in_m2(){
        let v = AdmData::new(2, Vec::default());
        let l1: Vec<AdmData>= vec![
            AdmData::new(4, Vec::new()),
            AdmData::new(5, Vec::new())
        ];
        let l2: VertexSet = vec![6,7].iter().cloned().collect();
        let mut u = AdmData::new(1, l1 );
        u.m1.insert(8, M1{ matching_partner : 6});
        u.m2.insert(6, M2{ matching_partner : 8, unmatched_r1_neighbours: VertexSet::default()});
        u.m1.insert(9, M1{ matching_partner : 7});
        u.m2.insert(7, M2{ matching_partner : 9, unmatched_r1_neighbours: VertexSet::default()});

        u.add_vertices_to_m(Rc::new(v), &l2, 5);

        assert_eq!(u.m2.get(&6).unwrap().unmatched_r1_neighbours.len(),1);
        assert_eq!(u.m2.get(&7).unwrap().unmatched_r1_neighbours.len(),1);
    }

    #[test]
    fn add_vertices_should_only_store_p_plus_1_unmatched_for_each_m2(){
        let v = AdmData::new(2, Vec::default());
        let l1: Vec<AdmData>= vec![
            AdmData::new(4, Vec::new()),
            AdmData::new(5, Vec::new())
        ];
        let l2: VertexSet = vec![6,7].iter().cloned().collect();
        let unmatched: VertexSet = vec![10,11,12].iter().cloned().collect();
        let mut u = AdmData::new(1, l1 );
        u.m2.insert(6, M2{ matching_partner : 8, unmatched_r1_neighbours: VertexSet::default()});
        u.m1.insert(9, M1{ matching_partner : 7});
        u.m2.insert(7, M2{ matching_partner : 9, unmatched_r1_neighbours: unmatched});

        u.add_vertices_to_m(Rc::new(v), &l2, 2);
        assert_eq!(u.m2.get(&7).unwrap().unmatched_r1_neighbours.len(),3);
    }

    #[test]
    fn add_vertices_should_add_v_to_m(){
        let v = AdmData::new(2, Vec::default());
        let l1: Vec<AdmData>= vec![
            AdmData::new(2, Vec::new()),
            AdmData::new(3, Vec::new())
        ];
        let l2: VertexSet = vec![4].iter().cloned().collect();
        let mut u = AdmData::new(1, l1 );

        u.add_vertices_to_m(Rc::new(v), &l2, 5);

        assert_eq!(u.m2.get(&4).unwrap().unmatched_r1_neighbours.len(),0);
        assert_eq!(u.m2.get(&4).unwrap().matching_partner,2);
        assert_eq!(u.m1.get(&2).unwrap().matching_partner,4);
    }

    #[test]
    fn remove_m2_should_remove_v_from_m_if_there_is_no_replacements(){
        let mut u = AdmData::new(
            1,
            vec![
                AdmData::new(2, Vec::new()),
                AdmData::new(3, Vec::new())
            ]
        );
        u.m2.insert(4, M2{ matching_partner : 5, unmatched_r1_neighbours: VertexSet::default()});
        u.m1.insert(5, M1{ matching_partner : 4});
        let v = AdmData::new(
            5,
            vec![
                AdmData::new(2, Vec::new()),
                AdmData::new(3, Vec::new())
            ]
        );
        u.r1.insert(5,Rc::new(v));
        u.remove_m2(&4,2);

        assert!(!u.m1.contains_key(&5));
        assert!(!u.m2.contains_key(&4));
    }

    #[test]
    fn remove_m2_should_replace_v_in_m_if_there_is_no_replacements(){
        let mut u = AdmData::new(
            1,
            vec![
                AdmData::new(2, Vec::new()),
                AdmData::new(3, Vec::new())
            ]
        );
        u.m2.insert(4, M2{ matching_partner : 5, unmatched_r1_neighbours: VertexSet::default()});
        u.m1.insert(5, M1{ matching_partner : 4});
        let v = AdmData::new(
            5,
            vec![
                AdmData::new(6, Vec::new()),
                AdmData::new(7, Vec::new())
            ]
        );
        u.r1.insert(5,Rc::new(v));
        u.remove_m2(&4,2);

        assert!(u.m1.contains_key(&5));
        assert!(!u.m2.contains_key(&4));
    }
}