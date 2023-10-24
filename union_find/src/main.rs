struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        let size = vec![1; n];
        UnionFind { parent, size }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

fn main() {
    let n = 5;
    let mut uf = UnionFind::new(n);

    uf.union(0, 1);
    uf.union(1, 2);
    uf.union(3, 4);

    println!("Are 0 and 2 connected? {}", uf.is_connected(0, 2));
    println!("Are 1 and 3 connected? {}", uf.is_connected(1, 3));

    uf.union(2, 3);

    println!("Are 0 and 2 connected? {}", uf.is_connected(0, 2));
    println!("Are 1 and 3 connected? {}", uf.is_connected(1, 3));
}
