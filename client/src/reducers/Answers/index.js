import {} from '../../constants/actionTypes'

const defaultState = {
  answer: {
    partOne: "part one answer",
    partTwo: "part two answer"
  },
  selectedDay: "1",
  selectedYear: "2018"
};

export default (state = defaultState, action) => {
  switch (action.type) {
    default:
      return state;
  }
};