use futures_cpupool::CpuPool;
use gotham::handler::NewHandler;
use gotham::middleware::pipeline::new_pipeline;
use gotham::router::Router;
use gotham::router::route::{Extractors, Route, RouteImpl, Delegation};
use gotham::router::route::dispatch::{new_pipeline_set, finalize_pipeline_set, PipelineSet,
                                      PipelineHandleChain, DispatcherImpl};
use gotham::router::route::matcher::MethodOnlyRouteMatcher;
use gotham::router::request::path::NoopPathExtractor;
use gotham::router::request::query_string::NoopQueryStringExtractor;
use gotham::router::response::finalizer::ResponseFinalizerBuilder;
use gotham::router::tree::TreeBuilder;
use gotham::router::tree::node::{NodeBuilder, SegmentType};
use hyper::Method;

use super::extractors::{AuthenticateQueryString, VerifyRequestPath};
use super::handler::{authenticate, verify};
use super::middleware::ThreadPoolMiddleware;

fn static_route<NH, P, C>(
    methods: Vec<Method>,
    new_handler: NH,
    active_pipelines: C,
    ps: PipelineSet<P>,
) -> Box<Route + Send + Sync>
where
    NH: NewHandler + 'static,
    C: PipelineHandleChain<P> + Send + Sync + 'static,
    P: Send + Sync + 'static,
{
    let matcher = MethodOnlyRouteMatcher::new(methods);
    let dispatcher = DispatcherImpl::new(new_handler, active_pipelines, ps);
    let extractors: Extractors<NoopPathExtractor, AuthenticateQueryString> = Extractors::new();
    let route = RouteImpl::new(
        matcher,
        Box::new(dispatcher),
        extractors,
        Delegation::Internal,
    );
    Box::new(route)
}

fn verify_route<NH, P, C>(
    methods: Vec<Method>,
    new_handler: NH,
    active_pipelines: C,
    pipeline_set: PipelineSet<P>,
) -> Box<Route + Send + Sync>
where
    NH: NewHandler + 'static,
    C: PipelineHandleChain<P> + Send + Sync + 'static,
    P: Send + Sync + 'static,
{
    let matcher = MethodOnlyRouteMatcher::new(methods);
    let dispatcher = DispatcherImpl::new(new_handler, active_pipelines, pipeline_set);

    let extractors: Extractors<VerifyRequestPath, NoopQueryStringExtractor> = Extractors::new();
    let route = RouteImpl::new(
        matcher,
        Box::new(dispatcher),
        extractors,
        Delegation::Internal,
    );
    Box::new(route)
}

pub fn router(cpu_pool: &CpuPool) -> Router {
    let mut tree_builder = TreeBuilder::new();

    let pool_middleware = ThreadPoolMiddleware::new(cpu_pool.clone());

    let ps_builder = new_pipeline_set();
    let (ps_builder, global) = ps_builder.add(
        new_pipeline()
            .add(pool_middleware)
            .build(),
    );
    let ps = finalize_pipeline_set(ps_builder);

    let mut seg_authenticate = NodeBuilder::new("authenticate", SegmentType::Static);
    seg_authenticate.add_route(static_route(vec![Method::Post],
                                        || Ok(authenticate),
                                        (global, ()),
                                        ps.clone()));
    tree_builder.add_child(seg_authenticate);

    let mut seg_verify = NodeBuilder::new("verify", SegmentType::Static);
    let mut seg_verify_jwt = NodeBuilder::new("jwt", SegmentType::Dynamic);
    seg_verify_jwt.add_route(verify_route(vec![Method::Post],
                                        || Ok(verify),
                                        (global, ()),
                                        ps.clone()));
    seg_verify.add_child(seg_verify_jwt);
    tree_builder.add_child(seg_verify);
    // Create a Node to represent the Request path /todo
    //let mut todo = NodeBuilder::new("todo", SegmentType::Static);

    /*todo.add_route(static_route(vec![Method::Post], // Use this Route for Get and Head Requests
                                        || Ok(verify),
                                        (global, ()), // This signifies that the active Pipelines for this route consist only of the global pipeline
                                        ps.clone())); // All the pipelines we've created for this Router*/


    let tree = tree_builder.finalize();

    let response_finalizer_builder = ResponseFinalizerBuilder::new();
    let response_finalizer = response_finalizer_builder.finalize();

    Router::new(tree, response_finalizer)
}
