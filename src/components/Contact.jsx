import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom'

class Contact extends React.Component {

    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div className='contact'>
                <div className='card'>
                    <div className='card-header'>
                        <h2>Contact Form</h2>
                    </div>
                    <div className='card-body'>
                        <p>
                            Send me any questions, comments, or complaints about anything. :D
                        </p>
                        <form action='mailto:duongquang1@gmail.com?subject=Blog%20Contact' method='post' encType='text/plain'>
                            <div className='row'>
                            <span>Name:</span>
                            <input name='name' type='text' /><br />
                            </div>
                            <div className='row'>
                            <span>Subject:</span>
                            <input name='subject' type='text' /><br />
                            </div>
                            <div className='row'>
                            <span>Comments:</span>
                            <textarea name='comment' /><br />
                            </div>
                            <div className='row'>
                            <input type='submit' value='Send'/>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        );
    }
}

export default Contact;
