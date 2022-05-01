use alfred_workflow_rust_project::alfred::{Alfred, AlfredEnv};
use alfred_workflow_rust_project::workflow::AlfredWorkflow;
use alfred_workflow_rust_project::workflow_item::WorkflowItem;

fn main() {
    let workflow = AlfredWorkflow::init();
    let cache_path = workflow.get_workflow_cache_path();
    let bundle_id = workflow.get_workflow_bundle_id();
    let uuid = workflow.get_workflow_uuid();
    let flow_name = workflow.get_workflow_name();
    let flow_version = workflow.get_workflow_version();
    let data_path = workflow.get_workflow_data_path();
    let app_version = workflow.get_version();
    let theme_text = workflow.get_theme_subtext();
    let version_build = workflow.get_version_build();
    let prefernce_path = workflow.get_preference_path();
    let theme = workflow.get_theme();
    let theme_background = workflow.get_theme_background();
    let theme_select_background = workflow.get_theme_selection_background();
    let test_config_key = workflow.get_config("test_config_var");

    workflow.add_item(WorkflowItem::new("cache_path")
        .subtitle(cache_path.as_str()))
        .add_item(WorkflowItem::new("bundle_id")
            .subtitle(bundle_id.as_str()))
        .add_item(WorkflowItem::new("uuid")
            .subtitle(uuid.as_str()))
        .add_item(WorkflowItem::new("flow_name")
            .subtitle(flow_name.as_str()))
        .add_item(WorkflowItem::new("flow_version")
            .subtitle(flow_version.as_str()))
        .add_item(WorkflowItem::new("data_path")
            .subtitle(data_path.as_str()))
        .add_item(WorkflowItem::new("app_version")
            .subtitle(app_version.as_str()))
        .add_item(WorkflowItem::new("theme_text")
            .subtitle(theme_text.as_str()))
        .add_item(WorkflowItem::new("version_build")
            .subtitle(version_build.as_str()))
        .add_item(WorkflowItem::new("preference_path")
            .subtitle(prefernce_path.as_str()))
        .add_item(WorkflowItem::new("theme")
            .subtitle(theme.as_str()))
        .add_item(WorkflowItem::new("theme_background")
            .subtitle(theme_background.as_str()))
        .add_item(WorkflowItem::new("theme_select_backgroud")
            .subtitle(theme_select_background.as_str()))
        .add_item(WorkflowItem::new("config_env")
            .subtitle(test_config_key.as_str()))
        .send_feedback();
}
