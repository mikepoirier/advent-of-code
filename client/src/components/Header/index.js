import React from 'react';
import {Link} from 'react-router-dom';

import './index.scss';

class Header extends React.Component {
  render() {
    return (
      <nav className="toolbar">
        <div className="container">
          <Link to="/" className="toolbar-title">
            {this.props.appName}
          </Link>
        </div>
      </nav>
    )
  }
}

export default Header