(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32)))
  (type (;7;) (func (param i32) (result i64)))
  (import "seal0" "seal_get_storage" (func (;0;) (type 5)))
  (import "seal0" "seal_set_storage" (func (;1;) (type 1)))
  (import "seal0" "seal_value_transferred" (func (;2;) (type 0)))
  (import "seal0" "seal_input" (func (;3;) (type 0)))
  (import "seal0" "seal_return" (func (;4;) (type 1)))
  (import "env" "memory" (memory (;0;) 2 16))
  (func (;5;) (type 2) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const -64
    i32.add
    local.tee 1
    global.set 0
    local.get 1
    i32.const 32
    i32.add
    local.get 0
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 1
    i32.const 24
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 1
    i64.const 0
    i64.store offset=40
    local.get 1
    local.get 0
    i64.load
    i64.store offset=8
    local.get 1
    i32.const 8
    i32.add
    call 6
    local.get 1
    i32.const 16384
    i32.store offset=52
    local.get 1
    i32.const 65572
    i32.store offset=48
    local.get 1
    i32.const 16384
    i32.store offset=56
    i32.const 65572
    local.get 1
    i32.const 56
    i32.add
    call 0
    local.set 0
    local.get 1
    i32.const 48
    i32.add
    local.get 1
    i32.load offset=56
    call 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            br_table 1 (;@3;) 0 (;@4;) 0 (;@4;) 2 (;@2;) 0 (;@4;)
          end
          unreachable
        end
        local.get 1
        local.get 1
        i64.load offset=48
        i64.store offset=56
        local.get 1
        i32.const 56
        i32.add
        call 8
        i32.const 255
        i32.and
        local.tee 0
        i32.const 2
        i32.ne
        br_if 1 (;@1;)
        unreachable
      end
      unreachable
    end
    local.get 1
    i32.const -64
    i32.sub
    global.set 0
    local.get 0
    i32.const 0
    i32.ne)
  (func (;6;) (type 2) (param i32) (result i32)
    (local i64 i64 i64)
    local.get 0
    i64.load offset=32
    local.set 1
    local.get 0
    i64.const 1
    i64.store offset=32
    local.get 0
    local.get 1
    local.get 0
    i64.load
    local.tee 2
    i64.add
    local.tee 1
    i64.store
    local.get 0
    local.get 0
    i64.load offset=8
    local.tee 3
    local.get 1
    local.get 2
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 1
    i64.store offset=8
    local.get 0
    local.get 0
    i64.load offset=16
    local.tee 2
    local.get 1
    local.get 3
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 1
    i64.store offset=16
    local.get 0
    local.get 0
    i64.load offset=24
    local.get 1
    local.get 2
    i64.lt_u
    i64.extend_i32_u
    i64.add
    i64.store offset=24
    local.get 0)
  (func (;7;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load offset=4
    local.set 3
    local.get 0
    i32.const 0
    i32.store offset=4
    local.get 0
    i32.load
    local.set 4
    local.get 0
    i32.const 65572
    i32.store
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    local.get 4
    local.get 3
    call 15
    local.get 0
    local.get 2
    i64.load offset=8
    i64.store align=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;8;) (type 2) (param i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    call 11
    local.get 1
    i32.load8_u offset=9
    local.set 0
    local.get 1
    i32.load8_u offset=8
    local.set 2
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    i32.const 2
    i32.const 1
    i32.const 2
    local.get 0
    i32.const 1
    i32.eq
    select
    i32.const 0
    local.get 0
    select
    local.get 2
    i32.const 1
    i32.and
    select)
  (func (;9;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const -64
    i32.add
    local.tee 2
    global.set 0
    local.get 2
    i32.const 32
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 24
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 16
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 2
    i64.const 0
    i64.store offset=40
    local.get 2
    local.get 1
    i64.load
    i64.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call 6
    local.get 2
    i32.const 56
    i32.add
    i32.const 16384
    i32.store
    local.get 2
    i32.const 65572
    i32.store offset=52
    local.get 2
    i32.const 0
    i32.store offset=48
    local.get 2
    local.get 2
    i32.const 48
    i32.add
    local.get 0
    call 10
    local.get 2
    i32.load
    local.get 2
    i32.load offset=4
    call 1
    local.get 2
    i32.const -64
    i32.sub
    global.set 0)
  (func (;10;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.tee 4
    i32.load
    local.set 5
    local.get 4
    i32.const 0
    i32.store
    local.get 1
    i32.load offset=4
    local.set 4
    local.get 1
    i32.const 65572
    i32.store offset=4
    local.get 3
    local.get 2
    i32.store8 offset=15
    local.get 3
    i32.const 1
    local.get 4
    local.get 5
    call 15
    block  ;; label = @1
      local.get 3
      i32.load offset=4
      i32.const 1
      i32.eq
      if  ;; label = @2
        local.get 3
        i32.load
        local.get 3
        i32.load8_u offset=15
        i32.store8
        local.get 1
        i32.const 0
        i32.store offset=8
        local.get 1
        i32.const 65572
        i32.store offset=4
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 5
        i32.const -1
        i32.add
        i32.store offset=8
        local.get 1
        local.get 4
        i32.const 1
        i32.add
        i32.store offset=4
        local.get 0
        i32.const 1
        i32.store offset=4
        local.get 0
        local.get 4
        i32.store
        local.get 3
        i32.const 16
        i32.add
        global.set 0
        return
      end
      unreachable
    end
    unreachable)
  (func (;11;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store8 offset=15
    block  ;; label = @1
      local.get 1
      local.get 2
      i32.const 15
      i32.add
      i32.const 1
      call 19
      i32.eqz
      if  ;; label = @2
        local.get 2
        i32.load8_u offset=15
        local.set 1
        br 1 (;@1;)
      end
      i32.const 1
      local.set 3
    end
    local.get 0
    local.get 1
    i32.store8 offset=1
    local.get 0
    local.get 3
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;12;) (type 7) (param i32) (result i64)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i64.const 0
    i64.store8 offset=12
    local.get 1
    i64.const 0
    i64.store32 offset=8
    block (result i32)  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.const 4
        i32.eq
        if  ;; label = @3
          local.get 1
          i64.load8_u offset=12
          i32.wrap_i64
          i32.const 4
          i32.lt_u
          local.set 2
          local.get 1
          i32.load offset=8
          br 2 (;@1;)
        end
        local.get 1
        local.get 0
        call 11
        local.get 1
        i32.load8_u
        i32.const 1
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 1
          i32.const 8
          i32.add
          local.get 2
          i32.add
          local.get 1
          i32.load8_u offset=1
          i32.store8
          local.get 1
          local.get 2
          i32.const 1
          i32.add
          local.tee 2
          i32.store8 offset=12
          br 1 (;@2;)
        end
      end
      local.get 2
      i32.const 255
      i32.and
      if  ;; label = @2
        local.get 1
        i32.const 0
        i32.store8 offset=12
      end
      i32.const 1
      local.set 2
      i32.const 0
    end
    local.set 0
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 2
    i64.extend_i32_u
    local.get 0
    i64.extend_i32_u
    i64.const 8
    i64.shl
    i64.or)
  (func (;13;) (type 3) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    i32.load
    i32.load8_u
    call 14
    unreachable)
  (func (;14;) (type 3) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 24
    i32.add
    i32.const 16384
    i32.store
    local.get 1
    i32.const 65572
    i32.store offset=20
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    call 10
    i32.const 0
    local.get 1
    i32.load offset=8
    local.get 1
    i32.load offset=12
    call 4
    unreachable)
  (func (;15;) (type 6) (param i32 i32 i32 i32)
    i32.const 0
    local.get 1
    i32.le_u
    if  ;; label = @1
      local.get 3
      local.get 1
      i32.ge_u
      if  ;; label = @2
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        local.get 2
        i32.store
        return
      end
      unreachable
    end
    unreachable)
  (func (;16;) (type 4) (result i32)
    i32.const 0
    call 17
    i32.const 255
    i32.and
    i32.const 2
    i32.shl
    i32.const 65536
    i32.add
    i32.load)
  (func (;17;) (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            if  ;; label = @5
              local.get 1
              i32.const 16384
              i32.store offset=12
              local.get 1
              i32.const 65572
              i32.store offset=8
              local.get 1
              i32.const 8
              i32.add
              call 20
              local.get 1
              local.get 1
              i64.load offset=8
              i64.store offset=16
              i32.const 1
              local.set 3
              local.get 1
              i32.const 16
              i32.add
              call 12
              local.tee 6
              i32.wrap_i64
              local.tee 5
              i32.const 1
              i32.and
              if  ;; label = @6
                i32.const 1
                local.set 2
                br 3 (;@3;)
              end
              local.get 6
              i64.const 1099511627775
              i64.and
              local.tee 6
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              local.set 0
              local.get 6
              i64.const 24
              i64.shr_u
              i32.wrap_i64
              local.set 4
              local.get 6
              i64.const 16
              i64.shr_u
              i32.wrap_i64
              local.set 2
              local.get 5
              i32.const 8
              i32.shr_u
              i32.const 255
              i32.and
              local.tee 5
              i32.const 30
              i32.ne
              if  ;; label = @6
                local.get 5
                i32.const 192
                i32.ne
                local.get 0
                i32.const 243
                i32.ne
                i32.or
                local.get 2
                i32.const 255
                i32.and
                i32.const 150
                i32.ne
                i32.or
                br_if 2 (;@4;)
                i32.const 0
                local.set 2
                local.get 4
                i32.const 255
                i32.and
                i32.const 165
                i32.eq
                br_if 3 (;@3;)
                br 2 (;@4;)
              end
              local.get 0
              i32.const 86
              i32.ne
              local.get 2
              i32.const 255
              i32.and
              i32.const 92
              i32.ne
              i32.or
              br_if 1 (;@4;)
              i32.const 0
              local.set 2
              i32.const 0
              local.set 3
              local.get 4
              i32.const 255
              i32.and
              i32.const 164
              i32.ne
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            local.get 1
            i32.const 16384
            i32.store offset=12
            local.get 1
            i32.const 65572
            i32.store offset=8
            local.get 1
            i32.const 8
            i32.add
            call 20
            local.get 1
            local.get 1
            i64.load offset=8
            i64.store offset=16
            i32.const 3
            local.set 2
            block  ;; label = @5
              local.get 1
              i32.const 16
              i32.add
              call 12
              local.tee 6
              i32.wrap_i64
              local.tee 4
              i32.const 1
              i32.and
              br_if 0 (;@5;)
              local.get 6
              i64.const 1099511627775
              i64.and
              local.tee 6
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              local.set 0
              local.get 6
              i64.const 24
              i64.shr_u
              i32.wrap_i64
              local.set 3
              local.get 6
              i64.const 16
              i64.shr_u
              i32.wrap_i64
              local.set 2
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.const 8
                  i32.shr_u
                  i32.const 255
                  i32.and
                  local.tee 4
                  i32.const 106
                  i32.ne
                  if  ;; label = @8
                    local.get 4
                    i32.const 209
                    i32.ne
                    local.get 0
                    i32.const 43
                    i32.ne
                    i32.or
                    local.get 2
                    i32.const 255
                    i32.and
                    i32.const 131
                    i32.ne
                    i32.or
                    br_if 2 (;@6;)
                    local.get 3
                    i32.const 255
                    i32.and
                    i32.const 81
                    i32.eq
                    br_if 1 (;@7;)
                    br 2 (;@6;)
                  end
                  local.get 0
                  i32.const 226
                  i32.ne
                  local.get 2
                  i32.const 255
                  i32.and
                  i32.const 55
                  i32.ne
                  i32.or
                  br_if 1 (;@6;)
                  i32.const 2
                  local.set 2
                  local.get 3
                  i32.const 255
                  i32.and
                  i32.const 18
                  i32.ne
                  br_if 1 (;@6;)
                  br 2 (;@5;)
                end
                i32.const 3
                local.set 2
                local.get 1
                i32.const 16
                i32.add
                call 8
                i32.const 255
                i32.and
                local.tee 0
                i32.const 2
                i32.eq
                br_if 1 (;@5;)
                local.get 0
                i32.const 0
                i32.ne
                local.set 2
                br 1 (;@5;)
              end
              i32.const 3
              local.set 2
            end
            i32.const 6
            local.get 2
            i32.const 3
            i32.eq
            local.tee 3
            br_if 2 (;@2;)
            drop
            i32.const 6
            local.get 2
            local.get 3
            select
            local.tee 2
            i32.const 2
            i32.eq
            if  ;; label = @5
              local.get 1
              i32.const 40
              i32.add
              i64.const 0
              i64.store
              local.get 1
              i32.const 32
              i32.add
              i64.const 0
              i64.store
              local.get 1
              i32.const 24
              i32.add
              i64.const 0
              i64.store
              local.get 1
              i64.const 0
              i64.store offset=16
              i32.const 0
              local.get 1
              i32.const 16
              i32.add
              call 9
              i32.const 8
              br 3 (;@2;)
            end
            local.get 1
            i32.const 40
            i32.add
            i64.const 0
            i64.store
            local.get 1
            i32.const 32
            i32.add
            i64.const 0
            i64.store
            local.get 1
            i32.const 24
            i32.add
            i64.const 0
            i64.store
            local.get 1
            i64.const 0
            i64.store offset=16
            local.get 2
            i32.const 1
            i32.and
            local.get 1
            i32.const 16
            i32.add
            call 9
            i32.const 8
            br 2 (;@2;)
          end
          i32.const 1
          local.set 2
          i32.const 1
          local.set 3
        end
        i32.const 6
        local.get 2
        br_if 0 (;@2;)
        drop
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.const 40
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 32
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 24
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i64.const 0
        i64.store offset=16
        local.get 1
        i32.const 16
        i32.add
        call 5
        i32.const 1
        i32.xor
        local.get 1
        i32.const 16
        i32.add
        call 9
        i32.const 8
      end
      local.get 1
      i32.const 48
      i32.add
      global.set 0
      return
    end
    local.get 1
    i32.const 40
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i32.const 32
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i32.const 24
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i64.const 0
    i64.store offset=16
    local.get 1
    local.get 1
    i32.const 16
    i32.add
    call 5
    i32.store8 offset=8
    local.get 1
    i32.const 8
    i32.add
    call 13
    unreachable)
  (func (;18;) (type 4) (result i32)
    (local i32 i32 i64 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 16384
    i32.store offset=4
    local.get 0
    i32.const 65572
    i32.store
    local.get 0
    i32.const 16384
    i32.store offset=16
    i32.const 65572
    local.get 0
    i32.const 16
    i32.add
    call 2
    local.get 0
    local.get 0
    i32.load offset=16
    call 7
    local.get 0
    local.get 0
    i64.load
    i64.store offset=8
    local.get 0
    i32.const 24
    i32.add
    local.tee 1
    i64.const 0
    i64.store
    local.get 0
    i64.const 0
    i64.store offset=16
    block (result i32)  ;; label = @1
      i32.const 1
      local.get 0
      i32.const 8
      i32.add
      local.get 0
      i32.const 16
      i32.add
      i32.const 16
      call 19
      br_if 0 (;@1;)
      drop
      local.get 1
      i64.load
      local.set 2
      local.get 0
      i64.load offset=16
      local.set 3
      i32.const 0
    end
    local.get 2
    local.get 3
    i64.or
    i64.eqz
    i32.eqz
    i32.or
    i32.eqz
    if  ;; label = @1
      i32.const 1
      call 17
      local.get 0
      i32.const 32
      i32.add
      global.set 0
      i32.const 255
      i32.and
      i32.const 2
      i32.shl
      i32.const 65536
      i32.add
      i32.load
      return
    end
    unreachable)
  (func (;19;) (type 5) (param i32 i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.get 2
      i32.ge_u
      if (result i32)  ;; label = @2
        local.get 1
        local.get 0
        i32.load
        local.get 2
        call 21
        local.get 0
        i32.load offset=4
        local.tee 1
        local.get 2
        i32.lt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i32.store offset=4
        local.get 0
        local.get 0
        i32.load
        local.get 2
        i32.add
        i32.store
        i32.const 0
      else
        i32.const 1
      end
      return
    end
    unreachable)
  (func (;20;) (type 3) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.load offset=4
    i32.store offset=12
    local.get 0
    i32.load
    local.get 1
    i32.const 12
    i32.add
    call 3
    local.get 0
    local.get 1
    i32.load offset=12
    call 7
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;21;) (type 1) (param i32 i32 i32)
    (local i32)
    loop (result i32)  ;; label = @1
      local.get 2
      local.get 3
      i32.eq
      if (result i32)  ;; label = @2
        local.get 0
      else
        local.get 0
        local.get 3
        i32.add
        local.get 1
        local.get 3
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        br 1 (;@1;)
      end
    end
    drop)
  (global (;0;) (mut i32) (i32.const 65536))
  (export "deploy" (func 16))
  (export "call" (func 18))
  (data (;0;) (i32.const 65536) "\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\08"))
