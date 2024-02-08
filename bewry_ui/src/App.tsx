import {
    DockviewDefaultTab,
    DockviewReact,
    DockviewReadyEvent,
    IDockviewPanelHeaderProps,
    IDockviewPanelProps,
    IDockviewHeaderActionsProps,
} from 'dockview';
import './App.scss'

const components = {
  default: (props: IDockviewPanelProps<{ title: string, widht: number }>) => {
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
            component: 'default',
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
    };

    return (
        <DockviewReact
            singleTabMode='fullwidth'
            components={components}
            onReady={onReady}
            className={props.theme || 'dockview-theme-abyss'}
            disableAutoResizing={true}
        />
    );
};

export default DockviewDemo;
