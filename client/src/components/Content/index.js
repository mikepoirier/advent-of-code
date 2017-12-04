import React from 'react';
import {connect} from 'react-redux';
import Answer from '../Answer'

const mapStateToProps = state => {
  return {
    answer: state.answers.answer,
    day: state.answers.selectedDay,
    year: state.answers.selectedYear
  }
};

const mapDispatchToProps = dispatch => ({

});

class App extends React.Component {

  render() {
    return (
      <div>
        <Answer selectedDay={this.props.day}
                selectedYear={this.props.year}
                answer={this.props.answer}/>
      </div>
    )
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(App)
