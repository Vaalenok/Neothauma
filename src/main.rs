mod primitives;
mod tests;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId}
};
use wgpu::{SurfaceConfiguration, RenderPassDescriptor, LoadOp, StoreOp};

// Состояние потока с lifetime
struct State<'a> {
    window: &'a Window, // Ссылка на окно
    surface: wgpu::Surface<'a>, // Поверхность отрисовки
    device: wgpu::Device, // GPU
    queue: wgpu::Queue, // Очередь выполнения
    config: SurfaceConfiguration, // Параметры отрисовки
    size: winit::dpi::PhysicalSize<u32> // Размеры окна
}

impl<'a> State<'a> {
    // Создание состояния (инициализация GPU)
    async fn new(window: &'a Window) -> Self {
        let size = window.inner_size(); // Текущий размер окна
        let instance = wgpu::Instance::default(); // Создание глобального контекста
        let surface = instance
            .create_surface(window)
            .expect("Не удалось создать поверхность"); // Создание поверхности

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Адаптер не найден"); // Поиск адаптера (GPU)

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("Не удалось создать логическое устройство"); // Создание логического устройства и очереди

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 120,
        }; // Создание конфигурации поверхности
        surface.configure(&device, &config); // Применение конфигурации

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size
        }
    }

    // Обновление конфигурации поверхности при изменении окна
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    // Отрисовка кадра
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?; // Получение текущего (возвращаемого) кадра
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default()); // Создание отображения возвращаемого кадра
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }); // Создание буфера команд для GPU

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: LoadOp::Clear(wgpu::Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        } // Параметры отрисовки

        self.queue.submit(Some(encoder.finish())); // Отправка кадра в очередь
        frame.present(); // Вынос кадра на экран
        Ok(())
    }
}

// Основное приложение
struct App<'a> {
    window: Option<Window>, // Настоящее окно
    state: Option<State<'a>> // Состояние
}

impl<'a> Default for App<'a> {
    // Инициализация приложения
    fn default() -> Self {
        Self {
            window: None,
            state: None
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    // Активация приложения
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_resizable(false)
            .with_title("Neothauma")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0)); // Атрибуты основного окна

        let window = Box::new(event_loop.create_window(attrs).unwrap()); // Создание основного окна
        let window_ref: &'static Window = Box::leak(window); // Ссылка на основное окно

        self.window = Some(unsafe { std::ptr::read(window_ref) }); // Сохранение состояния окна в приложении

        let state = pollster::block_on(State::new(window_ref)); // Состояние состояния

        self.state = Some(state); // Сохранение состояния GPU в приложении
    }

    // Обработка событий окна
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let Some(state) = &mut self.state {
            if id == state.window.id() {
                match event {
                    // Закрытие окна
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }
                    // Изменение размера окна
                    WindowEvent::Resized(size) => {
                        state.resize(size);
                        state.window.request_redraw();
                    }
                    // Изменение масштаба окна
                    WindowEvent::ScaleFactorChanged { .. } => {
                        state.window.request_redraw();
                    }
                    // Запрос на отрисовку
                    WindowEvent::RedrawRequested => {
                        if let Err(e) = state.render() {
                            eprintln!("Ошибка рендера: {:?}", e);
                        }
                    }
                    // Необработанный ивент
                    _ => {}
                }
            }
        }
    }

    // Деактивация приложения
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // Очистка состояния при сворачивании
        self.state = None;
    }
}

fn main() {
    // Создание цикла событий
    let event_loop = EventLoop::new().unwrap();

    // Создание и запуск приложения
    let mut app = App::default();

    // Запуск цикла обработки событий
    event_loop.run_app(&mut app).expect("Ошибка при запуске");
}
