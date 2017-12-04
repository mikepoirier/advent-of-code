import {APP_LOADED} from '../../constants/actionTypes'

const defaultState = {
  appName: 'Advent of Code',
  viewChangeCounter: 0
};

export default (state = defaultState, action) => {
  switch (action.type) {
    case APP_LOADED:
      return { ...state, viewChangeCounter: state.viewChangeCounter + 1};
    default:
      return state;
  }
};