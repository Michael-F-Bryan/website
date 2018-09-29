// import React, { Component } from "react";
//  
// export default class Resume extends Component {
//   render() {
//     return (
//       <div>
//         <h2>Resume</h2>
//           <object file="/public/resume.pdf" aria-label="My Resume" type="application/pdf" />
//         </div>
//       </div>
//     );
//   }
// }
// 

import React, { Component } from 'react';
import { Document, Page } from 'react-pdf/dist/entry.webpack';

export default class Resume extends Component {
  state = {
    numPages: null,
  }
 
  onDocumentLoad = ({ numPages }) => {
    this.setState({ numPages });
  }
 
  render() {
    const { numPages } = this.state;

    return (
      <div>
        <h2>Resume</h2>
        <Document file="/resume.pdf" onLoadSuccess={this.onDocumentLoad}>
          { 
            Array.from(
              new Array(numPages),
              (el, index) => (
                <Page key={`page_${index + 1}`} pageNumber={index + 1} />
              ),
            )
          }
        </Document>
      </div>
    );
  }
}
