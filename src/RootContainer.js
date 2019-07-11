/**
 * First visual component in the app.
 * It is the ancestor of all other screens and components.
 */
import React, { Component } from 'react';

// Dynamic import wasm module.
import('../wasm/pkg').then(wasm => {
  let result = wasm.scan('/Users/abhay/Dev/samples');
  console.log(result);
});

export default class RootContainer extends Component {
  render () {
    return (
      <div style={{ textAlign: 'center' }}>
        <h2>Reading file system operation not supported on wasm yet.</h2>
      </div>
    );
  }
}
