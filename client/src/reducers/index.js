import {combineReducers} from 'redux';
import {routerReducer} from 'react-router-redux';
import common from './Common';
import answers from './Answers'

export default combineReducers({
  common,
  answers,
  router: routerReducer
})