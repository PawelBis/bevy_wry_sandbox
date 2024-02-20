import {
    DockviewReact,
    DockviewReadyEvent,
    IDockviewPanelProps,
} from 'dockview';
import './App.scss'

const components = {
  default: (props: IDockviewPanelProps<{ title: string }>) => {
        return (
            <div
                style={{
                    height: '100%',
                    overflow: 'auto',
                    color: 'white',
                    position: 'relative',
                }}
            >
                <span
                    style={{
                        position: 'absolute',
                        top: '50%',
                        left: '50%',
                        transform: 'translate(-50%,-50%)',
                        pointerEvents: 'none',
                        fontSize: '42px',
                        opacity: 0.5,
                    }}
                >
                    {props.api.title}
                </span>
            </div>
        );
    },
  viewport: (_: IDockviewPanelProps<{}>) => {
        return (
            <div
                style={{
                    height: '100%',
                    overflow: 'auto',
                    color: 'white',
                    position: 'relative',
                }}
                className='viewport'
            >

            </div>
        );
    },
};

const DockviewDemo = (props: { theme?: string }) => {
    const onReady = (event: DockviewReadyEvent) => {
        const sceneTree = event.api.addPanel({
            id: 'panel_1',
            component: 'default',
            title: 'Scene tree',
        });

        const viewport = event.api.addPanel({
            id: 'viewport',
            component: 'viewport',
            title: 'viewport',
            position: {referencePanel: sceneTree, direction: 'right'},
        });

        const properties = event.api.addPanel({
            id: 'panel_2',
            component: 'default',
            title: 'Properties',
            position: { referencePanel: viewport, direction: 'right' },
        });


        const assetBrowser = event.api.addPanel({
          id: 'panel_3',
          component: 'default',
          title: 'Asset browser',
          position: {direction: 'below'},
        });

        const topPanel = event.api.addPanel({
            id: 'Top panel',
            component: 'default',
            title: 'Top panel',
            position: {direction:'above'}
        })

        sceneTree.api.setSize({width: 300});
        properties.api.setSize({width: 300});
        topPanel.api.setSize({height: 100});
        assetBrowser.api.setSize({height: 300});

        viewport.api.setActive();

        let websocket = new WebSocket("ws://localhost:8876");
        websocket.onopen = (_) => {

          viewport.api.onDidDimensionsChange((e) => {
            let xPosition = sceneTree.api.width;
            let yPosition = topPanel.api.height;
            let msg: any = {
              ResizeViewport: {
                new_position: {
                  x: xPosition,
                  y: yPosition,
                },
                new_size: {
                  x: e.width,
                  y: e.height,
                },
              }
            };
            websocket.send(JSON.stringify(msg));
          });
        };

        websocket.onmessage = (m) => {
          console.log(m)
        };

        websocket.onerror = (e) => console.log(e);

    };

    return (
        <DockviewReact
            singleTabMode='fullwidth'
            components={components}
            onReady={onReady}
            className={props.theme || 'dockview-theme-abyss'}
        />
    );
};

export default DockviewDemo;
