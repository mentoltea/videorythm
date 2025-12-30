function blur(image, radius)
    local width = image.width
    local height = image.height
    local result = Image(width/2, height/2)
    local time = frame_index / frame_count
    
    for x = 0, result.width - 1 do
        for y = 0, result.height - 1 do
            local r, g, b, a = 0.0, 0.0, 0.0, 255
            local samples = 0
            local avg = 0

            local outpixel = image:getPixel(2*x, 2*y)
            
            for dx = -1, 1 do
                for dy = -1, 1 do
                    if 2*x + dx >= 0 and 2*x + dx < image.width then
                        if 2*y + dy >= 0 and 2*y + dy < image.height then
                            local pixel = image:getPixel(2*x+dx, 2*y+dy)
                            avg = avg + pixel.r
                            avg = avg + pixel.g
                            avg = avg + pixel.b
                            samples = samples + 1
                        end
                    end
                end
            end
            
            avg = avg / ( 3 * samples)
            
            r = outpixel.r * time + avg * (1 - time)
            g = outpixel.g * time + avg * (1 - time)
            b = outpixel.b * time + avg * (1 - time)
            
            result:setPixel(x, y, Pixel(r, g, b, a))
        end
    end
    
    return result
end
    
return blur(original, 2)