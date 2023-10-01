use graphbench::graph::*;
use hopcroft_karp::matching;

pub struct AdmData {
    id: Vertex,
    pub layer_1: VertexSet,
    pub layer_2: VertexMap<VertexSet>,
    pub estimate: usize,
}

impl AdmData {
    pub fn new(id: Vertex, layer_1:VertexSet) -> Self{
        AdmData {id, estimate: layer_1.len(), layer_1, layer_2: VertexMap::default()}
    }

    // Compute packing for the layer 2 vertices
    // Return the maximum matching value
    pub fn compute_packing(&self) -> usize {
        let mut edges = Vec::default();
        let len_layer_2 = self.layer_2.len();
        if len_layer_2 <= 1 {
            return len_layer_2
        }
        for (u, vias) in &self.layer_2{
            edges.extend(vias.iter().map(|v| (u,v)));
        }
        matching(&edges).len()
    }

    // Update the estimate
    pub fn update_estimate(&mut self, p: usize) {
        //Don't update the estimate if estimate is not p
        if self.estimate != p{
            return;
        }
        self.estimate = self.layer_1.len() + self.compute_packing();
    }

    //Remove a layer 2 vertex
    pub fn update_indirect_neighbour(&mut self, u:&Vertex, p:usize){
        self.layer_2.remove(&u);
        //As we are losing a layer 2 vertex we want to reduce the estimate
        self.estimate = self.estimate.saturating_sub(1);
        self.update_estimate(p);
    }

    // Remove u from list of layer_1 vertices as it's is now to the right of AdmData
    // Adds all u_layer1 vertices to layer_2 with u being the via
    pub fn update_direct_neighbour(&mut self, u:&Vertex, u_layer1:&VertexSet, p:usize){
        // Remove the vertex that is now to the right
        self.layer_1.remove(u);
        self.estimate = self.estimate.saturating_sub(1);
        for v in u_layer1{
            // Only add v as a layer_2 vertex if it is not directly reachable
            if *v == self.id || self.layer_1.contains(v){
                continue
            }
            let entry = self.layer_2.entry(*v).or_default();
            entry.insert(*u);
        }
        self.update_estimate(p);
    }

}

#[cfg(test)]
mod test_adm_data {
    use graphbench::graph::{Vertex, VertexMap, VertexSet};
    use crate::admData::AdmData;

    #[test]
    fn update_estimate_does_not_update_if_estimate_is_not_p() {
        let layer_1 = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias = vec![7, 8, 9].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.layer_2.insert(6, vias);
        adm_data.update_estimate(3);
        assert_eq!(adm_data.estimate, 4);
        adm_data.update_estimate(5);
        assert_eq!(adm_data.estimate, 4);
    }

    #[test]
    fn update_estimate_updates_estimate_if_estimate_is_p() {
        let layer_1 = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias = vec![7, 8, 9].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.layer_2.insert(6, vias);
        adm_data.update_estimate(4);
        assert_eq!(adm_data.estimate, 5);
    }

    #[test]
    fn update_direct_neighbour_removes_u_from_layer_1_vertices(){
        let u : Vertex = 2;
        let layer_1 = vec![2, 3, 4, 5,6].iter().cloned().collect();
        let u_layer_1 = vec![7, 8].iter().cloned().collect();
        let expected_layer_1 = vec![3, 4, 5, 6].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 5;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        assert_eq!(adm_data.layer_1,expected_layer_1);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_direct_neighbour_adds_each_u_layer_1_to_layer_2_with_u_as_vias(){
        let u : Vertex = 2;
        let layer_1 = vec![2, 3, 4, 5].iter().cloned().collect();
        let u_layer_1: VertexSet = vec![6, 7, 8].iter().cloned().collect();
        let mut expected_layer_2 : VertexMap<VertexSet> = VertexMap::default();
        for v in &u_layer_1{
            let entry = expected_layer_2.entry(*v).or_default();
            entry.insert(u);
        }
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        assert_eq!(adm_data.layer_2,expected_layer_2);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_direct_neighbour_does_not_add_v_to_layer_2_if_v_is_directly_reachable(){
        let u : Vertex = 2;
        let layer_1: VertexSet = vec![2, 3, 4, 5].iter().cloned().collect();
        let u_layer_1: VertexSet = vec![5, 6, 7, 8].iter().cloned().collect();
        let expected_layer_2_u = u_layer_1.difference(&layer_1);
        let mut expected_layer_2 : VertexMap<VertexSet> = VertexMap::default();
        for v in expected_layer_2_u{
            let entry = expected_layer_2.entry(*v).or_default();
            entry.insert(u);
        }
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        assert_eq!(adm_data.layer_2,expected_layer_2);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_direct_neighbour_does_not_add_more_than_p_plus_1_vias(){
        let v : Vertex = 6;
        let u : Vertex = 11;
        let layer_1: VertexSet = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias: VertexSet = vec![7,8,9,10].iter().cloned().collect();
        let u_layer_1: VertexSet = vec![6].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.layer_2.insert(v,vias);
        adm_data.estimate = 5;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        let expected = adm_data.layer_2.get(&v).unwrap();
        assert_eq!(expected.len(),4);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_indirect_neighbour_removes_layer_2_vertex(){
        let v : Vertex = 6;
        let layer_1: VertexSet = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias: VertexSet = vec![7,8,9,10].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.layer_2.insert(v,vias);
        adm_data.estimate = 5;
        adm_data.update_indirect_neighbour(&v,3);
        assert_eq!(adm_data.layer_2.contains_key(&v),false);
        assert_eq!(adm_data.estimate,4);
    }
}