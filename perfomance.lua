function main(image)
    width = image.width
    height = image.height
    result = Image(width, height)
    for x = 0, width - 1 do
        for y = 0, height - 1 do
            -- Workload
            pixel = image:getPixel(x, y)
            avg = (pixel.r + pixel.g + pixel.b)/3
            result:setPixel(x, y, Pixel(avg, avg, avg, 255))
        end
        -- print(x + 1, "/", width)
    end
    return result
end

return main(original)