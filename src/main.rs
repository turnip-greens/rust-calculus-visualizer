use web_sys::HtmlInputElement;
use yew::prelude::*;
use wgpu::{Instance, Backends, InstanceDescriptor, PowerPreference, RequestAdapterOptions};

use std::sync::Arc;

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub compute_pipeline: wgpu::ComputePipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

pub async fn init_gpu() -> GpuContext {
    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("compute.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[Some(&bind_group_layout)],
        immediate_size: 0,
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        cache: None,
        compilation_options: wgpu::PipelineCompilationOptions::default()
    });

    GpuContext { device, queue, compute_pipeline, bind_group_layout }
}

fn clamp(value: &str, min: f32, max: f32) -> f32 {
    value.parse::<f32>().unwrap_or(min).clamp(min, max)
}

pub fn run_compute(ctx: &GpuContext, data_buffer: &wgpu::Buffer, workgroups_x: u32) {
    let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &ctx.bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: data_buffer.as_entire_binding(),
            },
        ],
    });

    let mut encoder = ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        
        // REUSE HAPPENS HERE: Set the pipeline you made during startup
        compute_pass.set_pipeline(&ctx.compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroups_x, 1, 1);
    }

    ctx.queue.submit(std::iter::once(encoder.finish()));
}


#[component]
fn App() -> Html {
    let samples = use_state(|| 1);
    let left_bounds = use_state(|| 0.0f32);
    let right_bounds = use_state(|| 1.0f32);
    let area_under_curve = use_state(|| 0.0f32);


    let on_slider_input = {
        let value = samples.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            value.set(clamp(&input.value(), 1.0, 128.0) as i32);
        })
    };

    let on_number_input = {
        let value = samples.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            value.set(clamp(&input.value(), 1.0, 128.0) as i32);
        })
    };

    let on_left_bounds_input = {
        let value = left_bounds.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            value.set(clamp(&input.value(), -f32::INFINITY, *right_bounds));
        })
    };
    
    let on_right_bounds_input = {
        let value = right_bounds.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            value.set(clamp(&input.value(), *left_bounds, f32::INFINITY));
        })
    };

    let run_compute = {
        let area = area_under_curve.clone();
        Callback::from(move |_: MouseEvent| {
            // Compute the area under the curve here
            area.set(0.0); // Replace with actual computation
        })
    };

    html! {
        <div>
            <p>{ format!("Samples: {} x 2048 = {}", *samples, *samples * 2048) }</p>
            <input
                type="number"
                min="1"
                max="128"
                step="1"
                value={(*samples).to_string()}
                oninput={on_number_input}
            />
            <input
                type="range"
                min="1"
                max="128"
                step="1"
                value={(*samples).to_string()}
                oninput={on_slider_input}
            />
            <p>{ format!("Area: {}", *area_under_curve) }</p>
            <button onclick={run_compute}>{"Compute Area"}</button>
            <input
                type="number"
                min="0"
                max="1"
                step="0.01"
                value={(*left_bounds).to_string()}
                oninput={on_left_bounds_input}
            />
            <input
                type="number"
                min="0"
                max="1"
                step="0.01"
                value={(*right_bounds).to_string()}
                oninput={on_right_bounds_input}
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}