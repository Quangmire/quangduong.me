import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect
} from 'react-router-dom'

class NotFound extends React.Component {

    render() {
        return (
            <div className='not-found'>
                <div className='card'>
                    <div className='card-header'>
                        <center>
                            <h2>404 Page Not Found</h2>
                        </center>
                    </div>
                    <div className='card-body'>
                        <center>
                            <p>
                                This page does not exist or wandered to a new home.
                            </p>
                            <p>
                                If you think this is an error, <Link to='/contact'>let me know</Link> so I can fix it! :D
                            </p>
                        </center>
                    </div>
                </div>
            </div>
        );
    }

};

export default NotFound;
