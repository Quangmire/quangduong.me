import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect
} from 'react-router-dom'

class WIP extends React.Component {

    render() {
        return (
            <div className='wip'>
                <div className='card'>
                    <div className='card-header'>
                        <center>
                            <h2>This page is under construction</h2>
                        </center>
                    </div>
                    <div className='card-body'>
                        <center>
                            <p>
                                This page does not exist yet.
                            </p>
                        </center>
                    </div>
                </div>
            </div>
        );
    }

};

export default WIP;
