import React from 'react';

const Answer = ({selectedDay, selectedYear, answer}) => {

  const createTitle = (day, year) => {
    const formattedDay = day.length < 2 ? `0${day}` : day;

    return `Dec. ${formattedDay}, ${year}`
  };

  return (
    <div className="answer-card">
      <h2 className="answer-title">{createTitle(selectedDay, selectedYear)}</h2>
      <div className="answer">
        <span>Part One: {answer.partOne}</span>
        </div>
      <div className="answer">
        <span>Part Two: {answer.partTwo}</span>
        </div>
    </div>
  )
};

export default Answer
