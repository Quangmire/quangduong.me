import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect
} from 'react-router-dom'
import {
    Card,
    CardBody,
    CardHeader
} from 'reactstrap';
import marked from 'marked';

class Page extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            title: '',
            text: '',
            404: false
        };
        marked.setOptions({
            renderer: new marked.Renderer(),
            gfm: true,
            tables: true,
            breaks: false,
            pedantic: false,
            sanitize: false,
            smartLists: true,
            smartypants: false
        });
    }

    componentDidMount() {
        return fetch('/projects/' + this.props.match.params.name +'.json', {method: 'GET'})
            .then(response => {
                if(response.ok) {
                    return response.json();
                } else {
                    this.state[404] = true;
                    return {title: '', text: []};
                }
            })
            .then(json => {
                this.setState({
                    title: json.title,
                    text: json.text.join('\n')
                });
            });
    }

    render() {
        if(this.state[404]) {
            return <Redirect push to='/404' />
        }
        return (
            <div className='project'>
                <Card>
                    <CardHeader>
                        <center>
                            <h3>{this.state.title}</h3>
                        </center>
                    </CardHeader>
                    <CardBody dangerouslySetInnerHTML={{__html: marked(this.state.text)}} />
                </Card>
            </div>
        );
    }

};

export default Page;
