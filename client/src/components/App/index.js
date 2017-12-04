import React from 'react';
import {connect} from 'react-redux';
import {APP_LOADED} from '../../constants/actionTypes';
import Header from '../Header'
import Content from '../Content'

const mapStateToProps = state => {
  return {
    appName: state.common.appName
  }
};

const mapDispatchToProps = dispatch => ({
  onLoad: () =>
    dispatch({type: APP_LOADED})
});

class App extends React.Component {

  componentWillMount() {
    this.props.onLoad()
  }

  render() {
    return (
      <div>
        <Header appName={this.props.appName}/>
        <Content/>
      </div>
    )
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(App)
