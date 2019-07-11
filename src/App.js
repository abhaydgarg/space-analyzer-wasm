/**
 * First component loaded after index.js. The purpose of this file
 * is to setup Redux or any other non-visual "global" modules.
 */
import React, { Component } from 'react';

import RootContainer from './RootContainer';

export default class App extends Component {
  render () {
    return <RootContainer />;
  }
}
