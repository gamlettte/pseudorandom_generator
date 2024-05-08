#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Seed {
    _multiplier: i32,
    _module: i32,
    _increase_val: i32,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Generator {
    _seed: Seed,
    _current_val: i32,
}

impl Generator {
    pub fn new(init_val: i32) -> Generator {
        return Generator {
            _current_val: init_val,
            _seed: Seed {
                _multiplier: 2_i32.pow(16),
                _module: 2_i32.pow(31),
                _increase_val: 28657,
            },
        };
    }

    pub fn get_random_value(&mut self) -> i32 {
        let result: i32;
        result = (self._current_val * self._seed._multiplier + self._seed._increase_val)
            % self._seed._module;
        self._current_val = result.clone();
        return result;
    }
}
