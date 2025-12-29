function blur(image, radius)
    local width = image.width
    local height = image.height
    local result = image:copy()
    local time = frame_index / frame_count
    
    for x = 0, width - 1 do
        for y = 0, height - 1 do
            local r, g, b, a = 0.0, 0.0, 0.0, 255
            local samples = 0
            local pixel = image:getPixel(x, y)
            r = pixel.r
            g = pixel.g
            b = pixel.b

            local avg = (r + g + b)/3
            
            result:setPixel(x, y, {
                r = r*time + avg*(1-time) ,
                g = g*time + avg*(1-time), 
                b = b*time + avg*(1-time),
                a = a 
            })
        end
    end
    
    return result
end
    
return blur(original, 2)