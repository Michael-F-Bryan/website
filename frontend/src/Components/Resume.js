import React, { Component } from "react";
 
export default class Resume extends Component {
  render() {
    return (
      <div className="container">
        <h2>Resume</h2>
          <div className="row embed-responsive embed-responsive-16by9" id="resume-preview">
              <object data="/resume.pdf" type="application/pdf">
                  <iframe title="resume" className="embed-responsive-item" src="/resume.pdf">
                      This browser does not support PDFs. Please download the PDF to view it:
                      <a href="/resume.pdf">Download PDF</a>
                  </iframe>
              </object>
          </div>
      </div>
    );
  }
}


