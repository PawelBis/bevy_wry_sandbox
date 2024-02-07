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
    viewport.params

        event.api.addPanel({
            id: 'panel_2',
            component: 'default',
            title: 'Properties',
            position: { referencePanel: viewport, direction: 'right' },
        });

        event.api.addPanel({
          id: 'panel_3',
          component: 'default',
          title: 'Asset browser',
          position: {direction: 'below'},
        });

        event.api.addPanel({
            id: 'Top panel',
            component: 'default',
            title: 'Top panel',
            position: {direction:'above'}
        })

        sceneTree.api.setActive();
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
