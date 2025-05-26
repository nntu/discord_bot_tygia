use poise::serenity_prelude::{AttachmentType, GatewayIntents, ChannelId};
use chrono::Local;
use dotenv::dotenv;
use image::ImageBuffer;
use screenshots::Screen;
use std::{env, path::Path, error::Error};
use log::{info, error, warn};

type Context<'a> = poise::Context<'a, (), Box<dyn Error + Send + Sync>>;

/// Lệnh !tygia - chụp màn hình và gửi kèm thời gian
#[poise::command(prefix_command, slash_command = false)]
async fn tygia(ctx: Context<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Nhận lệnh !tygia từ user {}", ctx.author().name);
    
    // Chụp màn hình
    info!("Đang chụp màn hình...");
    let screens = Screen::all()
        .map_err(|e| {
            error!("Không thể lấy thông tin màn hình: {}", e);
            format!("Không thể lấy thông tin màn hình: {}", e)
        })?;
    let screen = screens.get(0)
        .ok_or_else(|| {
            error!("Không tìm thấy màn hình");
            "Không tìm thấy màn hình"
        })?;
    
    let image = screen.capture()
        .map_err(|e| {
            error!("Không thể chụp màn hình: {}", e);
            format!("Không thể chụp màn hình: {}", e)
        })?;
    
    let width = image.width();
    let height = image.height();
    info!("Đã chụp màn hình với kích thước {}x{}", width, height);
    
    let samples = image.as_flat_samples();
    let rgba = samples.as_slice().to_vec();

    // Tạo buffer ảnh
    let buffer: ImageBuffer<image::Rgba<u8>, _> = ImageBuffer::from_raw(width, height, rgba)
        .ok_or_else(|| {
            error!("Không thể tạo buffer ảnh");
            "Không thể tạo buffer ảnh"
        })?;

    // Lưu ảnh tạm thời
    let path = Path::new("screenshot.png");
    info!("Đang lưu ảnh vào {}", path.display());
    buffer.save(path)?;

    // Lấy thời gian hiện tại
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    info!("Thời gian hiện tại: {}", now);

    // Gửi tin nhắn với ảnh
    info!("Đang gửi tin nhắn với ảnh...");
    ctx.send(|m| {
        m.content(format!("🕓 Tỷ giá lúc: `{}`", now))
         .attachment(AttachmentType::Path(path))
    })
    .await?;
    info!("Đã gửi tin nhắn thành công");

    // Xóa file ảnh tạm
    info!("Đang xóa file ảnh tạm...");
    std::fs::remove_file(path)?;
    info!("Đã xóa file ảnh tạm");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Khởi tạo logger
    env_logger::init();
    info!("Đang khởi động bot...");

    // Load biến môi trường từ file .env
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .map_err(|_| {
            error!("Không tìm thấy DISCORD_TOKEN trong file .env");
            "Không tìm thấy DISCORD_TOKEN trong file .env"
        })?;
    
    let channel_id = env::var("CHANNEL_ID")
        .map_err(|_| {
            error!("Không tìm thấy CHANNEL_ID trong file .env");
            "Không tìm thấy CHANNEL_ID trong file .env"
        })?;

    // Cấu hình bot
    info!("Đang cấu hình bot...");
    let options: poise::FrameworkOptions<(), Box<dyn Error + Send + Sync>> = poise::FrameworkOptions {
        commands: vec![tygia()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ..Default::default()
        },
        ..Default::default()
    };

    // Khởi động bot
    info!("Đang kết nối với Discord...");
    poise::Framework::builder()
        .options(options)
        .token(token)
        .intents(
            GatewayIntents::non_privileged() | 
            GatewayIntents::MESSAGE_CONTENT
        )
        .setup(move |ctx, _ready, _framework| {
            let channel_id = channel_id.clone();
            Box::pin(async move {
                info!("Bot đã sẵn sàng!");
                
                // Gửi tin nhắn thông báo khi bot khởi động
                if let Ok(channel_id) = channel_id.parse::<u64>() {
                    let channel = ChannelId(channel_id);
                    info!("Đang gửi tin nhắn thông báo đến channel {}", channel_id);
                    if let Err(e) = channel.send_message(&ctx.http, |m| {
                        m.content("🤖 Bot đã sẵn sàng! Sử dụng lệnh `!tygia` để chụp màn hình.")
                    }).await {
                        error!("Không thể gửi tin nhắn đến channel: {}", e);
                    } else {
                        info!("Đã gửi tin nhắn thông báo thành công");
                    }
                } else {
                    error!("Channel ID không hợp lệ: {}", channel_id);
                }
                
                Ok(())
            })
        })
        .run()
        .await?;

    Ok(())
}
