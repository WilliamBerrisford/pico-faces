#[derive(Debug, PartialEq)]
pub struct MetaEncoderState {
    current: EncoderState,
    last: EncoderState,
}

impl Default for MetaEncoderState {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaEncoderState {
    pub fn new() -> MetaEncoderState {
        MetaEncoderState {
            current: EncoderState::default(),
            last: EncoderState::default(),
        }
    }

    pub fn update_last(&mut self) {
        self.last = self.current
    }

    pub fn next(&mut self, clk: bool, dt: bool) {
        match self.current {
            EncoderState::Zero => {
                if clk && !dt {
                    self.current = EncoderState::Ninety
                } else if !clk && dt {
                    self.current = EncoderState::SevenTwenty
                } else {
                    self.current = EncoderState::Zero;
                    self.last = EncoderState::Zero;
                }
            }
            EncoderState::Ninety => {
                if clk && dt {
                    self.current = EncoderState::OneEighty
                } else if !clk && !dt {
                    self.current = EncoderState::Zero
                } else {
                    self.current = EncoderState::Zero;
                    self.last = EncoderState::Zero;
                }
            }
            EncoderState::OneEighty => {
                if !clk && dt {
                    self.current = EncoderState::SevenTwenty
                } else if clk && !dt {
                    self.current = EncoderState::Ninety
                } else {
                    self.current = EncoderState::Zero;
                    self.last = EncoderState::Zero;
                }
            }
            EncoderState::SevenTwenty => {
                if !clk && !dt {
                    self.current = EncoderState::Zero
                } else if clk && dt {
                    self.current = EncoderState::OneEighty
                } else {
                    self.current = EncoderState::Zero;
                    self.last = EncoderState::Zero;
                }
            }
        }
    }

    pub fn get_direction(&mut self) -> EncoderDirection {
        match self.current {
            EncoderState::Zero => match self.last {
                EncoderState::Ninety => EncoderDirection::Clockwise,
                EncoderState::SevenTwenty => EncoderDirection::AntiClockwise,
                _ => EncoderDirection::Bounce,
            },
            EncoderState::OneEighty => match self.last {
                EncoderState::Ninety => EncoderDirection::AntiClockwise,
                EncoderState::SevenTwenty => EncoderDirection::Clockwise,
                _ => EncoderDirection::Bounce,
            },
            _ => EncoderDirection::Bounce,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncoderDirection {
    Clockwise,
    AntiClockwise,
    Bounce,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncoderState {
    Zero,
    Ninety,
    OneEighty,
    SevenTwenty,
}

impl EncoderState {
    pub fn new() -> EncoderState {
        Self::Zero
    }
}

impl Default for EncoderState {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn default_state() {
    let meta_state = MetaEncoderState::default();
    assert_eq!(
        meta_state,
        MetaEncoderState {
            current: EncoderState::Zero,
            last: EncoderState::Zero,
        }
    );
}

#[test]
fn anti_clockwise() {
    let mut meta_state = MetaEncoderState::default();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                 ^
                 |

     */

    meta_state.next(true, false);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Bounce);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                      ^
                      |

     */

    meta_state.next(true, true);
    assert_eq!(meta_state.get_direction(), EncoderDirection::AntiClockwise);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                        ^
                        |

     */

    meta_state.next(false, true);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Bounce);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                          ^
                          |

     */

    meta_state.next(false, false);
    assert_eq!(meta_state.get_direction(), EncoderDirection::AntiClockwise);
    meta_state.update_last();
}

#[test]
fn clockwise() {
    let mut meta_state = MetaEncoderState::default();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                        ^
                        |

     */

    meta_state.next(false, true);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Bounce);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                      ^
                      |

     */

    meta_state.next(true, true);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Clockwise);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                    ^
                    |

     */

    meta_state.next(true, false);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Bounce);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                  ^
                  |

     */

    meta_state.next(false, false);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Clockwise);
    meta_state.update_last();
}

#[test]
fn repeat() {
    let mut meta_state = MetaEncoderState::default();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                        ^
                        |

     */

    meta_state.next(false, true);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Bounce);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                      ^
                      |

     */

    meta_state.next(true, false);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Bounce);
    meta_state.update_last();

    /*
          ____     ____     ____
    clk   |   |    |   |    |   |
        __|   |____|   |____|   |___

            ____     ____     ____
    dt      |   |    |   |    |   |
        ____|   |____|   |____|   |___

                        ^
                        |

     */

    meta_state.next(false, true);
    assert_eq!(meta_state.get_direction(), EncoderDirection::Bounce);
    meta_state.update_last();
}
